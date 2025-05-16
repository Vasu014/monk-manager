use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{debug, error};

use super::{AIClient, ModelConfig, Message as AIMessage};

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
    #[serde(rename = "system")]
    system_prompt: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Response {
    content: Vec<Content>,
}

#[derive(Debug, Deserialize, Serialize)]
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
            .timeout(std::time::Duration::from_secs(60))
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

    fn build_system_message(&self, project_context: Option<&str>) -> Message {
        let system_content = match project_context {
            Some(context) => format!(
                "You are an AI programming assistant. You're helping the user with their code project. Project context: {}",
                context
            ),
            None => "You are an AI programming assistant. You're helping the user with their code project.".to_string(),
        };
        
        Message {
            role: "assistant".to_string(),
            content: system_content,
        }
    }

    async fn send_request(&self, messages: Vec<Message>) -> Result<String> {
        let request = Request {
            model: self.config.model_name.clone(),
            system_prompt: "You are an AI programming assistant. You're helping the user with their code project.".to_string(),
            messages,
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
            anyhow::bail!("Anthropic API error: {}", error);
        }

        let response_text = response.text().await?;
        
        let response: Response = match serde_json::from_str(&response_text) {
            Ok(resp) => resp,
            Err(e) => {
                anyhow::bail!("Failed to parse Anthropic API response: {}", e)
            }
        };

        if response.content.is_empty() {
            anyhow::bail!("Empty content in Anthropic API response");
        }

        Ok(response.content[0].text.clone())
    }
}

#[async_trait]
impl AIClient for AnthropicClient {
    async fn explain(&self, code: &str, language: &str) -> Result<String> {
        let prompt = self.build_prompt(code, language);
        let messages = vec![
            Message {
                role: "user".to_string(),
                content: prompt,
            },
        ];

        self.send_request(messages).await
    }

    async fn chat(&self, messages: &[AIMessage], project_context: Option<&str>) -> Result<String> {
        let mut anthropic_messages = vec![self.build_system_message(project_context)];
        
        // Convert AIMessage to Anthropic Message format
        for message in messages {
            anthropic_messages.push(Message {
                role: message.role.clone(),
                content: message.content.clone(),
            });
        }

        self.send_request(anthropic_messages).await
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
            system_prompt: "You are an AI programming assistant. You're helping the user with their code project.".to_string(),
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
