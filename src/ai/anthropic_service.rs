use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{debug, error};

use super::{AIClient, ModelConfig};

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct Request {
    model: String,
    messages: Vec<Message>,
    max_tokens: usize,
    temperature: f32,
}

#[derive(Debug, Deserialize)]
struct Response {
    content: Vec<Content>,
}

#[derive(Debug, Deserialize)]
struct Content {
    text: String,
}

pub struct AnthropicClient {
    client: Client,
    config: ModelConfig,
}

impl AnthropicClient {
    pub fn new(config: ModelConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self { client, config })
    }

    fn build_prompt(&self, code: &str, language: &str) -> String {
        format!(
            "You are an expert programmer. Please explain the following {} code in a clear and concise way:\n\n```{}\n{}\n```",
            language,
            language,
            code
        )
    }
}

#[async_trait]
impl AIClient for AnthropicClient {
    async fn explain(&self, code: &str, language: &str) -> Result<String> {
        let prompt = self.build_prompt(code, language);
        debug!("Sending request to Anthropic API");

        let request = Request {
            model: self.config.model_name.clone(),
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt,
            }],
            max_tokens: self.config.max_tokens,
            temperature: self.config.temperature,
        };

        let response = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.config.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to Anthropic API")?;

        if !response.status().is_success() {
            let error = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            error!("Anthropic API error: {}", error);
            anyhow::bail!("Anthropic API error: {}", error);
        }

        let response: Response = response
            .json()
            .await
            .context("Failed to parse Anthropic API response")?;

        Ok(response.content[0].text.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{
        matchers::{header, method, path},
        Mock, MockServer, ResponseTemplate,
    };

    #[tokio::test]
    async fn test_explain_success() {
        let mock_server = MockServer::start().await;
        let config = ModelConfig {
            provider: "anthropic".to_string(),
            model_name: "claude-3-sonnet-20240229".to_string(),
            api_key: "test-key".to_string(),
            temperature: 0.7,
            max_tokens: 1000,
        };

        Mock::given(method("POST"))
            .and(path("/v1/messages"))
            .and(header("x-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(Response {
                content: vec![Content {
                    text: "This is a test explanation".to_string(),
                }],
            }))
            .mount(&mock_server)
            .await;

        let client = AnthropicClient {
            client: Client::new(),
            config,
        };

        let result = client.explain("fn main() {}", "rust").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "This is a test explanation");
    }

    #[tokio::test]
    async fn test_explain_error() {
        let mock_server = MockServer::start().await;
        let config = ModelConfig {
            provider: "anthropic".to_string(),
            model_name: "claude-3-sonnet-20240229".to_string(),
            api_key: "test-key".to_string(),
            temperature: 0.7,
            max_tokens: 1000,
        };

        Mock::given(method("POST"))
            .and(path("/v1/messages"))
            .and(header("x-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let client = AnthropicClient {
            client: Client::new(),
            config,
        };

        let result = client.explain("fn main() {}", "rust").await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("401"));
    }
} 