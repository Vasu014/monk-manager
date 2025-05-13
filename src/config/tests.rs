#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_config_serialization() {
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

        let yaml = serde_yaml::to_string(&config).unwrap();
        let deserialized: Config = serde_yaml::from_str(&yaml).unwrap();

        assert_eq!(deserialized.ai.provider, config.ai.provider);
        assert_eq!(deserialized.ai.model_name, config.ai.model_name);
        assert_eq!(deserialized.logging.level, config.logging.level);
        assert_eq!(deserialized.commands.default_language, config.commands.default_language);
    }

    #[test]
    fn test_config_file_creation() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("config.yaml");

        std::env::set_var("ANTHROPIC_API_KEY", "test-key");
        let config = Config::create_default_config(&config_path).unwrap();

        assert!(config_path.exists());
        assert_eq!(config.ai.provider, "anthropic");
        assert_eq!(config.ai.model_name, "claude-3-sonnet-20240229");
        assert_eq!(config.logging.level, "info");
        assert_eq!(config.commands.default_language, "rust");
    }

    #[test]
    fn test_config_loading() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("config.yaml");

        std::env::set_var("ANTHROPIC_API_KEY", "test-key");
        std::env::set_var("MONK_CONFIG", config_path.to_str().unwrap());

        let config = Config::load().unwrap();
        assert_eq!(config.ai.provider, "anthropic");
        assert_eq!(config.logging.level, "info");
    }
} 