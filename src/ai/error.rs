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
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[test]
    fn test_error_display() {
        let error = AIError::RequestError("test error".to_string());
        assert_eq!(error.to_string(), "API request failed: test error");

        let error = AIError::RateLimitExceeded;
        assert_eq!(error.to_string(), "Rate limit exceeded");

        let error = AIError::Timeout(Duration::from_secs(30));
        assert_eq!(error.to_string(), "Timeout: 30s");
    }

    #[tokio::test]
    async fn test_error_conversion() {
        // Test for 401 Unauthorized
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/test_401"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&server)
            .await;
        let response_401 = reqwest::get(format!("{}/test_401", server.uri()))
            .await
            .expect("Request to /test_401 should succeed initially");
        let reqwest_error_401 = response_401.error_for_status().expect_err("Expected HTTP error for 401");
        let ai_error_401: AIError = reqwest_error_401.into();
        assert!(matches!(ai_error_401, AIError::AuthenticationError(_)), "Expected AuthenticationError for 401, got {:?}", ai_error_401);

        // Test for 429 Too Many Requests
        Mock::given(method("GET"))
            .and(path("/test_429"))
            .respond_with(ResponseTemplate::new(429))
            .mount(&server)
            .await;
        let response_429 = reqwest::get(format!("{}/test_429", server.uri()))
            .await
            .expect("Request to /test_429 should succeed initially");
        let reqwest_error_429 = response_429.error_for_status().expect_err("Expected HTTP error for 429");
        let ai_error_429: AIError = reqwest_error_429.into();
        assert!(matches!(ai_error_429, AIError::RateLimitExceeded), "Expected RateLimitExceeded for 429, got {:?}", ai_error_429);

        // Test for a generic client error (e.g., 400 Bad Request)
        Mock::given(method("GET"))
            .and(path("/test_client_error"))
            .respond_with(ResponseTemplate::new(400)) // Bad Request
            .mount(&server)
            .await;
        let response_400 = reqwest::get(format!("{}/test_client_error", server.uri()))
            .await
            .expect("Request to /test_client_error should succeed initially");
        let reqwest_error_400 = response_400.error_for_status().expect_err("Expected HTTP error for 400");
        let ai_error_400: AIError = reqwest_error_400.into();
        assert!(matches!(ai_error_400, AIError::RequestError(_)), "Expected RequestError for 400, got {:?}", ai_error_400);

        // Test for server error
        Mock::given(method("GET"))
            .and(path("/test_server_error"))
            .respond_with(ResponseTemplate::new(500)) // Internal Server Error
            .mount(&server)
            .await;
        let response_500 = reqwest::get(format!("{}/test_server_error", server.uri()))
            .await
            .expect("Request to /test_server_error should succeed initially");
        let reqwest_error_500 = response_500.error_for_status().expect_err("Expected HTTP error for 500");
        let ai_error_500: AIError = reqwest_error_500.into();
        assert!(matches!(ai_error_500, AIError::RequestError(_)), "Expected RequestError for 500, got {:?}", ai_error_500);
    }
} 