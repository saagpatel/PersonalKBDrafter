use crate::error::AppError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct GenerateRequest {
    model: String,
    system: String,
    prompt: String,
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct GenerateResponse {
    response: String,
}

/// Check if Ollama is available at the given URL
pub async fn check_health(url: &str) -> Result<bool, AppError> {
    let endpoint = format!("{}/api/tags", url.trim_end_matches('/'));

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .map_err(|e| AppError::Internal(format!("Failed to create HTTP client: {}", e)))?;

    match client.get(&endpoint).send().await {
        Ok(response) => Ok(response.status().is_success()),
        Err(_) => Ok(false),
    }
}

/// Generate text using Ollama
pub async fn generate(
    url: &str,
    model: &str,
    system: &str,
    prompt: &str,
) -> Result<String, AppError> {
    let endpoint = format!("{}/api/generate", url.trim_end_matches('/'));

    let request_body = GenerateRequest {
        model: model.to_string(),
        system: system.to_string(),
        prompt: prompt.to_string(),
        stream: false,
    };

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .build()
        .map_err(|e| AppError::Internal(format!("Failed to create HTTP client: {}", e)))?;

    let response = client
        .post(&endpoint)
        .json(&request_body)
        .send()
        .await
        .map_err(|e| {
            if e.is_timeout() {
                AppError::Internal("Ollama request timed out after 120 seconds".to_string())
            } else if e.is_connect() {
                AppError::OllamaUnavailable { url: url.to_string() }
            } else {
                AppError::Network(e)
            }
        })?;

    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_default();

        // Check if it's a model not found error
        if error_text.contains("model") && error_text.contains("not found") {
            return Err(AppError::Internal(format!(
                "Model '{}' not found. Run 'ollama pull {}' to download it.",
                model, model
            )));
        }

        return Err(AppError::Internal(format!(
            "Ollama API error ({}): {}",
            status, error_text
        )));
    }

    let generate_response: GenerateResponse = response
        .json()
        .await
        .map_err(|e| AppError::Internal(format!("Failed to parse Ollama response: {}", e)))?;

    Ok(generate_response.response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires Ollama running
    async fn test_check_health() {
        let result = check_health("http://localhost:11434").await;
        // This will pass if Ollama is running, fail otherwise
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore] // Requires Ollama running with a model
    async fn test_generate() {
        let result = generate(
            "http://localhost:11434",
            "llama3.2",
            "You are a helpful assistant.",
            "Say 'test' and nothing else.",
        )
        .await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(!response.is_empty());
    }
}
