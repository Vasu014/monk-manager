use std::time::Duration;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AIError {
    #[error("API request failed: {0}")]
    RequestError(String),

    #[error("Invalid response from AI model: {0}")]
    InvalidResponse(String),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Timeout: {0:?}")]
    Timeout(Duration),

    #[error("Authentication failed: {0}")]
    AuthenticationError(String),

    #[error("Model error: {0}")]
    ModelError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),
}

impl From<reqwest::Error> for AIError {
    fn from(error: reqwest::Error) -> Self {
        if error.is_timeout() {
            AIError::Timeout(Duration::from_secs(30))
        } else if error.is_status() {
            match error.status() {
                Some(status) if status.is_client_error() => {
                    if status.as_u16() == 401 {
                        AIError::AuthenticationError("Invalid API key".to_string())
                    } else if status.as_u16() == 429 {
                        AIError::RateLimitExceeded
                    } else {
                        AIError::RequestError(format!("HTTP error: {}", status))
                    }
                }
                Some(status) if status.is_server_error() => {
                    AIError::RequestError(format!("Server error: {}", status))
                }
                _ => AIError::RequestError(error.to_string()),
            }
        } else {
            AIError::RequestError(error.to_string())
        }
    }
}

impl From<serde_json::Error> for AIError {
    fn from(error: serde_json::Error) -> Self {
        AIError::InvalidResponse(error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = AIError::RequestError("test error".to_string());
        assert_eq!(error.to_string(), "API request failed: test error");

        let error = AIError::RateLimitExceeded;
        assert_eq!(error.to_string(), "Rate limit exceeded");

        let error = AIError::Timeout(Duration::from_secs(30));
        assert_eq!(error.to_string(), "Timeout: 30s");
    }

    #[test]
    fn test_error_conversion() {
        let reqwest_error = reqwest::Error::builder()
            .status(401)
            .build();
        let ai_error: AIError = reqwest_error.into();
        assert!(matches!(ai_error, AIError::AuthenticationError(_)));

        let reqwest_error = reqwest::Error::builder()
            .status(429)
            .build();
        let ai_error: AIError = reqwest_error.into();
        assert!(matches!(ai_error, AIError::RateLimitExceeded));
    }
} 