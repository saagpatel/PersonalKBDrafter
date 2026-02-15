use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Jira error: {message} (HTTP {status})")]
    Jira { status: u16, message: String },

    #[error("Confluence error: {message} (HTTP {status})")]
    Confluence { status: u16, message: String },

    #[error("Token not configured for {service}")]
    TokenMissing { service: String },

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Ollama unavailable at {url}")]
    OllamaUnavailable { url: String },

    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("{0}")]
    Internal(String),
}

// Tauri requires errors to be serializable
impl Serialize for AppError {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct("AppError", 2)?;
        s.serialize_field("kind", &self.kind())?;
        s.serialize_field("message", &self.to_string())?;
        s.end()
    }
}

impl AppError {
    fn kind(&self) -> &'static str {
        match self {
            Self::Jira { .. } => "jira",
            Self::Confluence { .. } => "confluence",
            Self::TokenMissing { .. } => "token_missing",
            Self::Database(_) => "database",
            Self::OllamaUnavailable { .. } => "ollama_unavailable",
            Self::Network(_) => "network",
            Self::Internal(_) => "internal",
        }
    }
}

// Allow ? operator to work from r2d2 errors
impl From<r2d2::Error> for AppError {
    fn from(err: r2d2::Error) -> Self {
        AppError::Internal(format!("Database pool error: {}", err))
    }
}
