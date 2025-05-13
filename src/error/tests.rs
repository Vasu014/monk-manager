#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_error_conversion() {
        // Test IO error conversion
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let monk_error: MonkError = io_error.into();
        assert!(matches!(monk_error, MonkError::Io(_)));

        // Test Config error conversion
        let config_error = ConfigError::Missing("api_key".to_string());
        let monk_error: MonkError = config_error.into();
        assert!(matches!(monk_error, MonkError::Config(_)));

        // Test AI error conversion
        let ai_error = AIError::ModelError("Invalid response".to_string());
        let monk_error: MonkError = ai_error.into();
        assert!(matches!(monk_error, MonkError::AI(_)));

        // Test Command error conversion
        let command_error = CommandError::InvalidCommand("unknown".to_string());
        let monk_error: MonkError = command_error.into();
        assert!(matches!(monk_error, MonkError::Command(_)));
    }

    #[test]
    fn test_error_display() {
        let config_error = ConfigError::Missing("api_key".to_string());
        assert_eq!(
            config_error.to_string(),
            "Missing configuration: api_key"
        );

        let ai_error = AIError::RateLimitExceeded;
        assert_eq!(ai_error.to_string(), "Rate limit exceeded");

        let command_error = CommandError::InvalidArgument("--invalid".to_string());
        assert_eq!(
            command_error.to_string(),
            "Invalid argument value: --invalid"
        );
    }

    #[test]
    fn test_error_context() {
        let ai_error = AIError::Timeout(Duration::from_secs(30));
        assert_eq!(
            ai_error.to_string(),
            "Timeout: 30s"
        );
    }
} 