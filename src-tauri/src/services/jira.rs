use crate::error::AppError;
use crate::models::{JiraComment, JiraTicket};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde_json::Value;
use std::time::Duration;

pub struct JiraClient {
    base_url: String,
    pat: String,
    client: reqwest::Client,
}

impl JiraClient {
    pub fn new(base_url: String, pat: String) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(20))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());

        Self {
            base_url,
            pat,
            client,
        }
    }

    fn headers(&self) -> Result<HeaderMap, AppError> {
        let mut headers = HeaderMap::new();
        let auth_value = format!("Bearer {}", self.pat);
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&auth_value)
                .map_err(|e| AppError::Internal(format!("Invalid auth header: {}", e)))?,
        );
        Ok(headers)
    }

    pub async fn test_connection(&self) -> Result<bool, AppError> {
        let url = format!("{}/rest/api/2/myself", self.base_url);
        let response = self
            .client
            .get(&url)
            .headers(self.headers()?)
            .send()
            .await?;

        match response.status().as_u16() {
            200 => Ok(true),
            401 => Err(AppError::TokenMissing {
                service: "jira".to_string(),
            }),
            status => Err(AppError::Jira {
                status,
                message: "Connection test failed".to_string(),
            }),
        }
    }

    pub async fn get_ticket(&self, key: &str) -> Result<JiraTicket, AppError> {
        let url = format!(
            "{}/rest/api/2/issue/{}?fields=summary,description,status,priority,resolution,labels,components,comment,created,updated",
            self.base_url, key
        );

        let response = self
            .client
            .get(&url)
            .headers(self.headers()?)
            .send()
            .await?;

        let status_code = response.status().as_u16();

        if status_code == 404 {
            return Err(AppError::Jira {
                status: 404,
                message: format!("Ticket {} not found", key),
            });
        }

        if status_code == 403 {
            return Err(AppError::Jira {
                status: 403,
                message: "No permission to view this ticket".to_string(),
            });
        }

        if status_code != 200 {
            return Err(AppError::Jira {
                status: status_code,
                message: "Failed to fetch ticket".to_string(),
            });
        }

        let json: Value = response.json().await?;
        self.parse_ticket(&json)
    }

    pub async fn search_tickets(&self, query: &str) -> Result<Vec<JiraTicket>, AppError> {
        // Validate input to prevent JQL injection
        if query.contains('"') || query.contains('\'') || query.contains('\\') {
            return Err(AppError::Internal(
                "Search query contains invalid characters. Please avoid quotes and backslashes.".to_string()
            ));
        }

        let jql = format!("text ~ \"{}\"", query);
        let url = format!(
            "{}/rest/api/2/search?jql={}&maxResults=20&fields=summary,status,updated",
            self.base_url,
            urlencoding::encode(&jql)
        );

        let response = self
            .client
            .get(&url)
            .headers(self.headers()?)
            .send()
            .await?;

        let status_code = response.status().as_u16();
        if status_code != 200 {
            return Err(AppError::Jira {
                status: status_code,
                message: "Search failed".to_string(),
            });
        }

        let json: Value = response.json().await?;
        let issues = json["issues"].as_array().ok_or_else(|| {
            AppError::Internal("Invalid search response: missing issues array".to_string())
        })?;

        let mut tickets = Vec::new();
        for issue in issues {
            if let Ok(ticket) = self.parse_ticket(issue) {
                tickets.push(ticket);
            }
        }

        Ok(tickets)
    }

    fn parse_ticket(&self, json: &Value) -> Result<JiraTicket, AppError> {
        let key = json["key"]
            .as_str()
            .ok_or_else(|| AppError::Internal("Missing ticket key".to_string()))?
            .to_string();

        let fields = &json["fields"];

        let summary = fields["summary"]
            .as_str()
            .ok_or_else(|| AppError::Internal("Missing summary".to_string()))?
            .to_string();

        let description = fields["description"].as_str().map(|s| s.to_string());

        let status = fields["status"]["name"]
            .as_str()
            .unwrap_or("Unknown")
            .to_string();

        let priority = fields["priority"]["name"]
            .as_str()
            .map(|s| s.to_string());

        let resolution = fields["resolution"]["name"]
            .as_str()
            .map(|s| s.to_string());

        let labels = fields["labels"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let components = fields["components"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v["name"].as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let comments = self.parse_comments(&fields["comment"]);

        let created = fields["created"]
            .as_str()
            .unwrap_or("")
            .to_string();

        let updated = fields["updated"]
            .as_str()
            .unwrap_or("")
            .to_string();

        Ok(JiraTicket {
            key,
            summary,
            description,
            status,
            priority,
            resolution,
            labels,
            components,
            comments,
            created,
            updated,
        })
    }

    fn parse_comments(&self, comment_data: &Value) -> Vec<JiraComment> {
        let comments_array = match comment_data["comments"].as_array() {
            Some(arr) => arr,
            None => return Vec::new(),
        };

        comments_array
            .iter()
            .filter_map(|comment| {
                let author = comment["author"]["displayName"]
                    .as_str()?
                    .to_string();
                let body = comment["body"].as_str()?.to_string();
                let created = comment["created"].as_str()?.to_string();

                Some(JiraComment {
                    author,
                    body,
                    created,
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ticket() {
        let client = JiraClient::new("http://test".to_string(), "token".to_string());

        let json = serde_json::json!({
            "key": "TEST-123",
            "fields": {
                "summary": "Test issue",
                "description": "Test description",
                "status": { "name": "Open" },
                "priority": { "name": "High" },
                "resolution": { "name": "Fixed" },
                "labels": ["bug", "urgent"],
                "components": [{ "name": "Backend" }],
                "comment": {
                    "comments": [{
                        "author": { "displayName": "John Doe" },
                        "body": "Test comment",
                        "created": "2024-01-01T00:00:00.000Z"
                    }]
                },
                "created": "2024-01-01T00:00:00.000Z",
                "updated": "2024-01-02T00:00:00.000Z"
            }
        });

        let ticket = client.parse_ticket(&json).unwrap();
        assert_eq!(ticket.key, "TEST-123");
        assert_eq!(ticket.summary, "Test issue");
        assert_eq!(ticket.status, "Open");
        assert_eq!(ticket.labels, vec!["bug", "urgent"]);
        assert_eq!(ticket.comments.len(), 1);
        assert_eq!(ticket.comments[0].author, "John Doe");
    }
}
