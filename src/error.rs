use thiserror::Error;

#[derive(Error, Debug)]
pub enum MonkError {
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),

    #[error("AI model error: {0}")]
    AI(#[from] AIError),

    #[error("Command error: {0}")]
    Command(#[from] CommandError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Missing configuration: {0}")]
    Missing(String),

    #[error("Invalid configuration value: {0}")]
    Invalid(String),

    #[error("Failed to load configuration: {0}")]
    LoadError(String),
}

#[derive(Error, Debug)]
pub enum AIError {
    #[error("Model error: {0}")]
    ModelError(String),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Invalid response format: {0}")]
    InvalidResponse(String),

    #[error("Timeout: {0:?}")]
    Timeout(std::time::Duration),
}

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("Invalid command: {0}")]
    InvalidCommand(String),

    #[error("Missing required argument: {0}")]
    MissingArgument(String),

    #[error("Invalid argument value: {0}")]
    InvalidArgument(String),
} 