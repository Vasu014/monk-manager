use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::File,
    path::{Path, PathBuf},
};
use tracing::debug;

use crate::ai::ModelConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub ai: ModelConfig,
    pub logging: LoggingConfig,
    pub commands: CommandsConfig,
    pub security: SecurityConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
    pub output: String,
    pub file: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandsConfig {
    pub default_language: String,
    pub default_format: String,
    pub timeout: u64,
    pub explain: ExplainConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplainConfig {
    pub max_context_lines: usize,
    pub language_detection: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub secrets_file: Option<PathBuf>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::find_config_file()?;
        debug!("Loading configuration from: {:?}", config_path);

        let config = match config_path.extension().and_then(|ext| ext.to_str()) {
            Some("toml") => Self::load_toml(&config_path)?,
            Some("json") => Self::load_json(&config_path)?,
            Some("yaml") | Some("yml") => Self::load_yaml(&config_path)?,
            _ => anyhow::bail!("Unsupported configuration file format"),
        };

        // Apply environment variable overrides
        let config = config.apply_env_overrides()?;
        config.validate()?;

        Ok(config)
    }

    fn find_config_file() -> Result<PathBuf> {
        // First check environment variable
        if let Ok(path) = env::var("MONK_CONFIG") {
            let path = PathBuf::from(path);
            if path.exists() {
                return Ok(path);
            }
        }

        // Then check standard locations
        let config_dirs = vec![
            env::current_dir()?,
            dirs::config_dir().unwrap_or_else(|| PathBuf::from(".")),
            PathBuf::from("."),
        ];

        let config_names = vec!["monk.toml", "monk.json", "monk.yaml", "monk.yml"];

        for dir in config_dirs {
            for name in &config_names {
                let path = dir.join(name);
                if path.exists() {
                    return Ok(path);
                }
            }
        }

        // If no config found, create default
        let default_path = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("monk-manager")
            .join("config.yaml");
        
        Self::create_default_config(&default_path)?;
        Ok(default_path)
    }

    fn load_toml(path: &Path) -> Result<Self> {
        let contents = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {:?}", path))?;
        toml::from_str(&contents)
            .with_context(|| format!("Failed to parse TOML config: {:?}", path))
    }

    fn load_json(path: &Path) -> Result<Self> {
        let file = File::open(path)
            .with_context(|| format!("Failed to open config file: {:?}", path))?;
        serde_json::from_reader(file)
            .with_context(|| format!("Failed to parse JSON config: {:?}", path))
    }

    fn load_yaml(path: &Path) -> Result<Self> {
        let file = File::open(path)
            .with_context(|| format!("Failed to open config file: {:?}", path))?;
        serde_yaml::from_reader(file)
            .with_context(|| format!("Failed to parse YAML config: {:?}", path))
    }

    fn apply_env_overrides(mut self) -> Result<Self> {
        if let Ok(api_key) = env::var("ANTHROPIC_API_KEY") {
            self.ai.api_key = api_key;
        }

        if let Ok(level) = env::var("MONK_LOG_LEVEL") {
            self.logging.level = level;
        }

        Ok(self)
    }

    fn validate(&self) -> Result<()> {
        if self.ai.api_key.is_empty() {
            anyhow::bail!("AI API key is required");
        }

        if self.ai.max_tokens == 0 {
            anyhow::bail!("Max tokens must be greater than 0");
        }

        if !(0.0..=1.0).contains(&self.ai.temperature) {
            anyhow::bail!("Temperature must be between 0.0 and 1.0");
        }

        Ok(())
    }

    fn create_default_config(path: &Path) -> Result<Self> {
        let config = Config {
            ai: ModelConfig {
                provider: "anthropic".to_string(),
                model_name: "claude-3-5-haiku-20241022".to_string(),
                api_key: env::var("ANTHROPIC_API_KEY")?,
                temperature: 0.7,
                max_tokens: 1024,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                format: "pretty".to_string(),
                output: "stderr".to_string(),
                file: None,
            },
            commands: CommandsConfig {
                default_language: "rust".to_string(),
                default_format: "markdown".to_string(),
                timeout: 30,
                explain: ExplainConfig {
                    max_context_lines: 10,
                    language_detection: true,
                },
            },
            security: SecurityConfig {
                secrets_file: None,
            },
        };

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, serde_yaml::to_string(&config)?)?;

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_config_validation() {
        let config = Config {
            ai: ModelConfig {
                provider: "anthropic".to_string(),
                model_name: "claude-3-5-haiku-20241022".to_string(),
                api_key: "test-key".to_string(),
                temperature: 0.7,
                max_tokens: 1000,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                format: "pretty".to_string(),
                output: "stderr".to_string(),
                file: None,
            },
            commands: CommandsConfig {
                default_language: "rust".to_string(),
                default_format: "markdown".to_string(),
                timeout: 30,
                explain: ExplainConfig {
                    max_context_lines: 10,
                    language_detection: true,
                },
            },
            security: SecurityConfig {
                secrets_file: None,
            },
        };

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validation_failure() {
        let config = Config {
            ai: ModelConfig {
                provider: "anthropic".to_string(),
                model_name: "claude-3-5-haiku-20241022".to_string(),
                api_key: "".to_string(),
                temperature: 1.5,
                max_tokens: 0,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                format: "pretty".to_string(),
                output: "stderr".to_string(),
                file: None,
            },
            commands: CommandsConfig {
                default_language: "rust".to_string(),
                default_format: "markdown".to_string(),
                timeout: 30,
                explain: ExplainConfig {
                    max_context_lines: 10,
                    language_detection: true,
                },
            },
            security: SecurityConfig {
                secrets_file: None,
            },
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_loading() {
        let temp_file = NamedTempFile::new().unwrap();
        let config_str = r#"
            ai:
              provider: anthropic
              model_name: claude-3-5-haiku-20241022
              api_key: test-key
              max_tokens: 1000
              temperature: 0.7

            logging:
              level: info
              format: pretty
              output: stderr

            commands:
              default_language: rust
              default_format: markdown
              timeout: 30
              explain:
                max_context_lines: 10
                language_detection: true

            security:
              secrets_file: null
        "#;
        std::fs::write(&temp_file, config_str).unwrap();

        let config = Config::load_yaml(temp_file.path()).unwrap();
        assert_eq!(config.ai.model_name, "claude-3-5-haiku-20241022");
        assert_eq!(config.ai.api_key, "test-key");
        assert_eq!(config.ai.max_tokens, 1000);
        assert_eq!(config.ai.temperature, 0.7);
    }
} 