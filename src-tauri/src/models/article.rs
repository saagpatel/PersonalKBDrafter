use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/bindings/")]
pub struct Article {
    pub id: i64,
    #[ts(optional)]
    pub ticket_key: Option<String>,
    pub title: String,
    pub problem: String,
    pub solution: String,
    #[ts(optional)]
    pub expected_result: Option<String>,
    #[ts(optional)]
    pub prerequisites: Option<String>,
    #[ts(optional)]
    pub additional_notes: Option<String>,
    pub tags: Vec<String>,
    pub content_markdown: String,
    pub status: ArticleStatus,
    #[ts(optional)]
    pub confluence_page_id: Option<String>,
    #[ts(optional)]
    pub confluence_url: Option<String>,
    #[ts(optional)]
    pub confluence_space_key: Option<String>,
    #[ts(optional)]
    pub quality_score: Option<u8>,
    #[ts(optional)]
    pub template_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/bindings/")]
pub enum ArticleStatus {
    Draft,
    Published,
}

impl ArticleStatus {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "draft" => Ok(ArticleStatus::Draft),
            "published" => Ok(ArticleStatus::Published),
            _ => Err(format!("Invalid status: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewArticle {
    pub ticket_key: Option<String>,
    pub title: String,
    pub problem: String,
    pub solution: String,
    pub expected_result: Option<String>,
    pub prerequisites: Option<String>,
    pub additional_notes: Option<String>,
    pub tags: Vec<String>,
    pub content_markdown: String,
    pub template_id: Option<String>,
}
