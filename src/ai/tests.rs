#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use mockall::mock;

    mock! {
        AIModel {
            fn generate_response(&self, prompt: &str) -> Result<String>;
        }
    }

    #[tokio::test]
    async fn test_model_config_creation() {
        let config = ModelConfig {
            provider: "anthropic".to_string(),
            model_name: "test-model".to_string(),
            api_key: "test-key".to_string(),
            temperature: 0.7,
            max_tokens: 1000,
        };

        let model = config.create_model();
        assert!(model.is_ok());
    }

    #[tokio::test]
    async fn test_unsupported_provider() {
        let config = ModelConfig {
            provider: "unsupported".to_string(),
            model_name: "test-model".to_string(),
            api_key: "test-key".to_string(),
            temperature: 0.7,
            max_tokens: 1000,
        };

        let model = config.create_model();
        assert!(model.is_err());
    }

    #[tokio::test]
    async fn test_model_response() {
        let mut mock_model = MockAIModel::new();
        mock_model
            .expect_generate_response()
            .with(eq("test prompt"))
            .returning(|_| Ok("test response".to_string()));

        let response = mock_model.generate_response("test prompt").await;
        assert!(response.is_ok());
        assert_eq!(response.unwrap(), "test response");
    }
} 