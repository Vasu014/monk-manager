# Error Handling Specification

## Overview
Monk Manager implements a comprehensive error handling system that provides clear, actionable error messages while maintaining detailed error context for debugging.

## Error Types

### Base Error Type
```rust
#[derive(Debug, thiserror::Error)]
pub enum MonkError {
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),
    
    #[error("AI model error: {0}")]
    AI(#[from] AIError),
    
    #[error("Command error: {0}")]
    Command(#[from] CommandError),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Validation error: {0}")]
    Validation(String),
}
```

### Domain-Specific Errors
```rust
#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error("Invalid command: {0}")]
    InvalidCommand(String),
    
    #[error("Missing required argument: {0}")]
    MissingArgument(String),
    
    #[error("Invalid argument value: {0}")]
    InvalidArgument(String),
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Missing configuration: {0}")]
    Missing(String),
    
    #[error("Invalid configuration value: {0}")]
    Invalid(String),
    
    #[error("Failed to load configuration: {0}")]
    LoadError(String),
}
```

## Error Context

### Error Context Structure
```rust
#[derive(Debug)]
pub struct ErrorContext {
    pub error_type: String,
    pub message: String,
    pub source: Option<Box<dyn Error + Send + Sync>>,
    pub backtrace: Option<Backtrace>,
    pub metadata: HashMap<String, String>,
}
```

### Context Builder
```rust
impl ErrorContext {
    pub fn new(error_type: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            error_type: error_type.into(),
            message: message.into(),
            source: None,
            backtrace: None,
            metadata: HashMap::new(),
        }
    }
    
    pub fn with_source(mut self, source: impl Error + Send + Sync + 'static) -> Self {
        self.source = Some(Box::new(source));
        self
    }
    
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}
```

## Error Handling Patterns

### Result Type
```rust
pub type Result<T> = std::result::Result<T, MonkError>;
```

### Error Conversion
```rust
impl From<std::io::Error> for MonkError {
    fn from(error: std::io::Error) -> Self {
        MonkError::Io(error)
    }
}
```

### Error Propagation
```rust
fn process_command(input: &str) -> Result<()> {
    let command = parse_command(input)?;
    let result = execute_command(command)?;
    Ok(result)
}
```

## Error Reporting

### User-Facing Errors
```rust
impl MonkError {
    pub fn user_message(&self) -> String {
        match self {
            MonkError::Config(e) => format!("Configuration error: {}", e),
            MonkError::AI(e) => format!("AI model error: {}", e),
            MonkError::Command(e) => format!("Command error: {}", e),
            MonkError::Io(e) => format!("IO error: {}", e),
            MonkError::Validation(e) => format!("Validation error: {}", e),
        }
    }
}
```

### Debug Information
```rust
impl MonkError {
    pub fn debug_info(&self) -> String {
        format!("{:#?}", self)
    }
}
```

## Error Recovery

### Retry Logic
```rust
pub async fn with_retry<F, T, E>(f: F, max_retries: usize) -> Result<T>
where
    F: Fn() -> Future<Output = Result<T, E>>,
    E: Into<MonkError>,
{
    let mut retries = 0;
    loop {
        match f().await {
            Ok(result) => return Ok(result),
            Err(e) if retries < max_retries => {
                retries += 1;
                continue;
            }
            Err(e) => return Err(e.into()),
        }
    }
}
```

### Fallback Behavior
```rust
impl Command {
    pub async fn execute_with_fallback(&self) -> Result<()> {
        match self.execute().await {
            Ok(result) => Ok(result),
            Err(e) => self.fallback().await,
        }
    }
}
```

## Error Logging

### Structured Logging
```rust
impl MonkError {
    pub fn log_error(&self) {
        error!(
            error_type = %self.error_type(),
            message = %self.user_message(),
            backtrace = ?self.backtrace,
            "Error occurred"
        );
    }
}
```

### Error Metrics
```rust
pub struct ErrorMetrics {
    pub error_count: Counter,
    pub error_types: HashMap<String, Counter>,
}

impl ErrorMetrics {
    pub fn record_error(&self, error: &MonkError) {
        self.error_count.increment(1);
        self.error_types
            .entry(error.error_type())
            .or_insert_with(Counter::new)
            .increment(1);
    }
}
```

## Testing

### Error Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_conversion() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let monk_error: MonkError = io_error.into();
        assert!(matches!(monk_error, MonkError::Io(_)));
    }

    #[test]
    fn test_error_context() {
        let context = ErrorContext::new("TestError", "Test message")
            .with_metadata("key", "value");
        assert_eq!(context.error_type, "TestError");
        assert_eq!(context.message, "Test message");
        assert_eq!(context.metadata.get("key"), Some(&"value".to_string()));
    }
} 