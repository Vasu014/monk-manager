#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use mockall::predicate::*;
    use mockall::mock;
    use clap::Parser;

    mock! {
        AIModel {
            fn generate_response(&self, prompt: &str) -> Result<String>;
        }
    }

    #[tokio::test]
    async fn test_explain_command() {
        let config = Config {
            ai: ModelConfig {
                provider: "anthropic".to_string(),
                model_name: "test-model".to_string(),
                api_key: "test-key".to_string(),
                temperature: 0.7,
                max_tokens: 1000,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                format: "pretty".to_string(),
                output: "stderr".to_string(),
            },
            commands: CommandsConfig {
                default_language: "rust".to_string(),
                default_format: "markdown".to_string(),
                timeout: 30,
            },
        };

        let args = ExplainArgs {
            input: "What is a closure?".to_string(),
            language: Some("rust".to_string()),
            detail: DetailLevel::Medium,
        };

        let result = execute(args, &config).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_detail_level_parsing() {
        assert_eq!("basic".parse::<DetailLevel>().unwrap(), DetailLevel::Basic);
        assert_eq!("medium".parse::<DetailLevel>().unwrap(), DetailLevel::Medium);
        assert_eq!("detailed".parse::<DetailLevel>().unwrap(), DetailLevel::Detailed);
        assert!("invalid".parse::<DetailLevel>().is_err());
    }

    #[test]
    fn test_explain_command_parsing() {
        let args = vec!["monkexplain", "path/to/file.rs"];
        let explain_args = explain::ExplainArgs::parse_from(args);
        assert_eq!(explain_args.file, "path/to/file.rs");
        assert_eq!(explain_args.language, None);
    }

    #[test]
    fn test_explain_command_with_language() {
        let args = vec![
            "monkexplain",
            "path/to/file.rs",
            "--language",
            "rust",
        ];
        let explain_args = explain::ExplainArgs::parse_from(args);
        assert_eq!(explain_args.file, "path/to/file.rs");
        assert_eq!(explain_args.language, Some("rust".to_string()));
    }

    #[test]
    fn test_interactive_mode_is_default() {
        let args = vec!["monk-manager"];
        let cli = Cli::parse_from(args);
        assert!(cli.command.is_none());
    }
} 