use crate::error::AppError;
use crate::models::confluence::{ConfluenceSpace, PublishResult};
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub struct ConfluenceClient {
    base_url: String,
    pat: String,
    client: reqwest::Client,
}

#[derive(Debug, Serialize)]
struct CreatePageRequest {
    #[serde(rename = "type")]
    page_type: String,
    title: String,
    space: SpaceKey,
    body: Body,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<Metadata>,
}

#[derive(Debug, Serialize)]
struct UpdatePageRequest {
    version: Version,
    title: String,
    #[serde(rename = "type")]
    page_type: String,
    body: Body,
}

#[derive(Debug, Serialize)]
struct SpaceKey {
    key: String,
}

#[derive(Debug, Serialize)]
struct Body {
    storage: Storage,
}

#[derive(Debug, Serialize)]
struct Storage {
    value: String,
    representation: String,
}

#[derive(Debug, Serialize)]
struct Metadata {
    labels: Vec<Label>,
}

#[derive(Debug, Serialize)]
struct Label {
    prefix: String,
    name: String,
}

#[derive(Debug, Serialize)]
struct Version {
    number: i32,
}

#[derive(Debug, Deserialize)]
struct SpacesResponse {
    results: Vec<SpaceResult>,
}

#[derive(Debug, Deserialize)]
struct SpaceResult {
    key: String,
    name: String,
}

#[derive(Debug, Deserialize)]
struct PageResponse {
    id: String,
    #[serde(rename = "_links")]
    links: Links,
    version: VersionInfo,
}

#[derive(Debug, Deserialize)]
struct Links {
    base: String,
    webui: String,
}

#[derive(Debug, Deserialize)]
struct VersionInfo {
    number: i32,
}

impl ConfluenceClient {
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

    /// Test connection to Confluence
    pub async fn test_connection(&self) -> Result<bool, AppError> {
        let endpoint = format!(
            "{}/rest/api/content?limit=1",
            self.base_url.trim_end_matches('/')
        );

        let response = self
            .client
            .get(&endpoint)
            .header("Authorization", format!("Bearer {}", self.pat))
            .send()
            .await?;

        Ok(response.status().is_success())
    }

    /// List available Confluence spaces
    pub async fn list_spaces(&self) -> Result<Vec<ConfluenceSpace>, AppError> {
        let endpoint = format!(
            "{}/rest/api/space?limit=100&type=global",
            self.base_url.trim_end_matches('/')
        );

        let response = self
            .client
            .get(&endpoint)
            .header("Authorization", format!("Bearer {}", self.pat))
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::Confluence {
                status: status.as_u16(),
                message: error_text,
            });
        }

        let spaces_response: SpacesResponse = response.json().await?;

        Ok(spaces_response
            .results
            .into_iter()
            .map(|s| ConfluenceSpace {
                key: s.key,
                name: s.name,
            })
            .collect())
    }

    /// Create a new page in Confluence
    pub async fn create_page(
        &self,
        space_key: &str,
        title: &str,
        body_xhtml: &str,
        labels: &[String],
    ) -> Result<PublishResult, AppError> {
        let endpoint = format!(
            "{}/rest/api/content",
            self.base_url.trim_end_matches('/')
        );

        let metadata = if !labels.is_empty() {
            Some(Metadata {
                labels: labels
                    .iter()
                    .map(|l| Label {
                        prefix: "global".to_string(),
                        name: l.clone(),
                    })
                    .collect(),
            })
        } else {
            None
        };

        let request_body = CreatePageRequest {
            page_type: "page".to_string(),
            title: title.to_string(),
            space: SpaceKey {
                key: space_key.to_string(),
            },
            body: Body {
                storage: Storage {
                    value: body_xhtml.to_string(),
                    representation: "storage".to_string(),
                },
            },
            metadata,
        };

        let response = self
            .client
            .post(&endpoint)
            .header("Authorization", format!("Bearer {}", self.pat))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();

            // Check for specific error conditions
            if status.as_u16() == 401 {
                return Err(AppError::Confluence {
                    status: 401,
                    message: "Authentication failed. Check your Confluence PAT.".to_string(),
                });
            } else if status.as_u16() == 403 {
                return Err(AppError::Confluence {
                    status: 403,
                    message: format!("No write access to space '{}'. Check permissions.", space_key),
                });
            } else if status.as_u16() == 409 {
                return Err(AppError::Confluence {
                    status: 409,
                    message: format!("A page titled '{}' already exists in this space.", title),
                });
            }

            return Err(AppError::Confluence {
                status: status.as_u16(),
                message: error_text,
            });
        }

        let page_response: PageResponse = response.json().await?;

        Ok(PublishResult {
            page_id: page_response.id,
            url: format!("{}{}", page_response.links.base, page_response.links.webui),
            space_key: space_key.to_string(),
        })
    }

    /// Get space key for a Confluence page
    pub async fn get_page_space_key(&self, page_id: &str) -> Result<String, AppError> {
        let endpoint = format!(
            "{}/rest/api/content/{}?expand=space",
            self.base_url.trim_end_matches('/'),
            page_id
        );

        let response = self
            .client
            .get(&endpoint)
            .header("Authorization", format!("Bearer {}", self.pat))
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::Confluence {
                status: status.as_u16(),
                message: error_text,
            });
        }

        let json: serde_json::Value = response.json().await?;
        let space_key = json["space"]["key"]
            .as_str()
            .ok_or_else(|| AppError::Internal("Failed to get space key from page".to_string()))?
            .to_string();

        Ok(space_key)
    }

    /// Get current version of a Confluence page
    pub async fn get_page_version(&self, page_id: &str) -> Result<i32, AppError> {
        let endpoint = format!(
            "{}/rest/api/content/{}?expand=version",
            self.base_url.trim_end_matches('/'),
            page_id
        );

        let response = self
            .client
            .get(&endpoint)
            .header("Authorization", format!("Bearer {}", self.pat))
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::Confluence {
                status: status.as_u16(),
                message: error_text,
            });
        }

        let page_response: PageResponse = response.json().await?;
        Ok(page_response.version.number)
    }

    /// Update an existing page in Confluence
    pub async fn update_page(
        &self,
        page_id: &str,
        title: &str,
        body_xhtml: &str,
        current_version: i32,
    ) -> Result<PublishResult, AppError> {
        let endpoint = format!(
            "{}/rest/api/content/{}",
            self.base_url.trim_end_matches('/'),
            page_id
        );

        let request_body = UpdatePageRequest {
            version: Version {
                number: current_version + 1,
            },
            title: title.to_string(),
            page_type: "page".to_string(),
            body: Body {
                storage: Storage {
                    value: body_xhtml.to_string(),
                    representation: "storage".to_string(),
                },
            },
        };

        let response = self
            .client
            .put(&endpoint)
            .header("Authorization", format!("Bearer {}", self.pat))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::Confluence {
                status: status.as_u16(),
                message: error_text,
            });
        }

        let page_response: PageResponse = response.json().await?;

        // Fetch the page with space information
        let space_key = self.get_page_space_key(&page_response.id).await?;

        Ok(PublishResult {
            page_id: page_response.id,
            url: format!("{}{}", page_response.links.base, page_response.links.webui),
            space_key,
        })
    }
}
