# Configuration Specification

## Overview
Monk Manager uses a flexible configuration system that supports multiple configuration sources and formats, with environment variable overrides and secure credential management.

## Configuration Structure

### Base Configuration
```rust
#[derive(Debug, Deserialize)]
pub struct Config {
    pub ai: AIConfig,
    pub logging: LoggingConfig,
    pub commands: CommandsConfig,
    pub security: SecurityConfig,
}

#[derive(Debug, Deserialize)]
pub struct AIConfig {
    pub model: String,
    pub api_key: SecretString,
    pub temperature: f32,
    pub max_tokens: usize,
}

#[derive(Debug, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
    pub output: String,
}

#[derive(Debug, Deserialize)]
pub struct CommandsConfig {
    pub default_language: String,
    pub default_format: String,
    pub timeout: Duration,
}

#[derive(Debug, Deserialize)]
pub struct SecurityConfig {
    pub rate_limit: RateLimitConfig,
    pub allowed_commands: Vec<String>,
}
```

## Configuration Sources

### File-Based Configuration
```rust
impl Config {
    pub fn from_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config = match path.extension().and_then(|ext| ext.to_str()) {
            Some("toml") => toml::from_str(&content)?,
            Some("json") => serde_json::from_str(&content)?,
            Some("yaml") | Some("yml") => serde_yaml::from_str(&content)?,
            _ => return Err(ConfigError::UnsupportedFormat.into()),
        };
        Ok(config)
    }
}
```

### Environment Variables
```rust
impl Config {
    pub fn from_env() -> Result<Self> {
        let config = Config {
            ai: AIConfig {
                model: env::var("MONK_AI_MODEL")?,
                api_key: SecretString::new(env::var("MONK_AI_API_KEY")?),
                temperature: env::var("MONK_AI_TEMPERATURE")?.parse()?,
                max_tokens: env::var("MONK_AI_MAX_TOKENS")?.parse()?,
            },
            // ... other config fields
        };
        Ok(config)
    }
}
```

## Configuration Management

### Configuration Builder
```rust
impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}

#[derive(Default)]
pub struct ConfigBuilder {
    config: Config,
}

impl ConfigBuilder {
    pub fn with_ai_config(mut self, ai_config: AIConfig) -> Self {
        self.config.ai = ai_config;
        self
    }
    
    pub fn with_logging_config(mut self, logging_config: LoggingConfig) -> Self {
        self.config.logging = logging_config;
        self
    }
    
    pub fn build(self) -> Config {
        self.config
    }
}
```

### Configuration Validation
```rust
impl Config {
    pub fn validate(&self) -> Result<()> {
        if self.ai.temperature < 0.0 || self.ai.temperature > 1.0 {
            return Err(ConfigError::InvalidValue("temperature".into()).into());
        }
        
        if self.ai.max_tokens == 0 {
            return Err(ConfigError::InvalidValue("max_tokens".into()).into());
        }
        
        Ok(())
    }
}
```

## Secure Configuration

### Secret Management
```rust
#[derive(Debug, Clone)]
pub struct SecretString(String);

impl SecretString {
    pub fn new(value: String) -> Self {
        Self(value)
    }
    
    pub fn expose(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for SecretString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "********")
    }
}
```

### Credential Storage
```rust
pub struct CredentialStore {
    store: Keyring,
}

impl CredentialStore {
    pub fn new() -> Result<Self> {
        Ok(Self {
            store: Keyring::new("monk-manager")?,
        })
    }
    
    pub fn store_credential(&self, key: &str, value: &str) -> Result<()> {
        self.store.set_password(key, value)?;
        Ok(())
    }
    
    pub fn get_credential(&self, key: &str) -> Result<SecretString> {
        let value = self.store.get_password(key)?;
        Ok(SecretString::new(value))
    }
}
```

## Configuration Updates

### Hot Reloading
```rust
pub struct ConfigManager {
    config: Arc<RwLock<Config>>,
    watcher: notify::RecommendedWatcher,
}

impl ConfigManager {
    pub fn new(config: Config) -> Result<Self> {
        let config = Arc::new(RwLock::new(config));
        let watcher = notify::RecommendedWatcher::new(
            move |res: Result<Event, _>| {
                if let Ok(event) = res {
                    // Handle config file changes
                }
            },
        )?;
        
        Ok(Self { config, watcher })
    }
    
    pub async fn update_config(&self, new_config: Config) -> Result<()> {
        let mut config = self.config.write().await;
        *config = new_config;
        Ok(())
    }
}
```

## Testing

### Configuration Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_validation() {
        let config = Config {
            ai: AIConfig {
                model: "gpt-4".into(),
                api_key: SecretString::new("test-key".into()),
                temperature: 0.7,
                max_tokens: 1000,
            },
            // ... other fields
        };
        
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_builder() {
        let config = Config::builder()
            .with_ai_config(AIConfig {
                model: "gpt-4".into(),
                api_key: SecretString::new("test-key".into()),
                temperature: 0.7,
                max_tokens: 1000,
            })
            .build();
            
        assert_eq!(config.ai.model, "gpt-4");
    }
}
``` 