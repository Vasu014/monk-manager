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
    pub repository_home: Option<String>,
    #[serde(skip)] // Don't serialize this path to the config file itself
    pub config_file_path: Option<PathBuf>,
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

        let mut config_with_path = config;
        config_with_path.config_file_path = Some(config_path);

        Ok(config_with_path)
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
                api_key: env::var("ANTHROPIC_API_KEY").unwrap_or_else(|_| "YOUR_ANTHROPIC_API_KEY_HERE".to_string()),
                temperature: 0.7,
                max_tokens: 1024,
                api_base_url: None,
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
            repository_home: None,
            config_file_path: Some(path.to_path_buf()),
        };

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, serde_yaml::to_string(&config)?)?;

        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let path = self.config_file_path.as_ref().ok_or_else(|| anyhow::anyhow!("Config file path not set, cannot save."))?;
        debug!("Saving configuration to: {:?}", path);

        if let Some(parent) = path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)
                    .with_context(|| format!("Failed to create config directory: {:?}", parent))?;
            }
        }

        let contents = match path.extension().and_then(|ext| ext.to_str()) {
            Some("toml") => toml::to_string_pretty(self)?,
            Some("json") => serde_json::to_string_pretty(self)?,
            Some("yaml") | Some("yml") => serde_yaml::to_string(self)?,
            _ => anyhow::bail!("Unsupported configuration file format for saving: {:?}", path.extension()),
        };
        
        std::fs::write(path, contents)
            .with_context(|| format!("Failed to write config file: {:?}", path))?;
        
        Ok(())
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
            repository_home: None,
            config_file_path: None,
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
            repository_home: None,
            config_file_path: None,
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_loading() {
        // Ensure the temp file has a .yaml extension for Config::load() to work
        let temp_file = NamedTempFile::with_suffix(".yaml").unwrap();
        // The api_key in the string below will be loaded first.
        let config_str = r#"
            ai:
              provider: anthropic
              model_name: claude-3-5-haiku-20241022
              api_key: "api_key_from_file_content"
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
            repository_home: "/test/repo/home"
        "#;
        std::fs::write(temp_file.path(), config_str).unwrap();

        // Unset any potentially interfering env var first, then set specifically for this test.
        std::env::remove_var("ANTHROPIC_API_KEY"); 
        let expected_api_key_from_env = "env_api_key_for_load_test_specific";
        std::env::set_var("ANTHROPIC_API_KEY", expected_api_key_from_env);
        std::env::set_var("MONK_CONFIG", temp_file.path().to_str().unwrap());

        let config = Config::load().unwrap();
        assert_eq!(config.ai.model_name, "claude-3-5-haiku-20241022");
        // This assertion should now pass as apply_env_overrides will use the env var set in this test.
        assert_eq!(config.ai.api_key, expected_api_key_from_env); 
        assert_eq!(config.ai.max_tokens, 1000);
        assert_eq!(config.ai.temperature, 0.7);
        assert_eq!(config.repository_home.as_deref(), Some("/test/repo/home"));
        assert!(config.config_file_path.is_some(), "config_file_path should be set by Config::load()");
        assert_eq!(config.config_file_path.as_ref().unwrap(), temp_file.path());

        std::env::remove_var("MONK_CONFIG");
        std::env::remove_var("ANTHROPIC_API_KEY");
    }

    #[test]
    fn test_default_config_creation_and_save() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let config_path = temp_dir.path().join("monk-manager").join("config.yaml");

        // Set required env var for default config creation
        std::env::set_var("ANTHROPIC_API_KEY", "test_api_key_for_default_config");

        // Create default config (implicitly tests population of repository_home=None and config_file_path)
        let mut config = Config::create_default_config(&config_path)?;
        assert!(config.repository_home.is_none());
        assert_eq!(config.config_file_path.as_ref(), Some(&config_path));

        // Modify and save
        config.repository_home = Some("/new/repo/path".to_string());
        config.save()?;

        // Load and verify
        let loaded_config = Config::load_yaml(&config_path)?; // Directly load to avoid find_config_file logic for this test
        assert_eq!(loaded_config.repository_home.as_deref(), Some("/new/repo/path"));
        
        std::env::remove_var("ANTHROPIC_API_KEY"); // Clean up env var
        temp_dir.close()?;
        Ok(())
    }
} 