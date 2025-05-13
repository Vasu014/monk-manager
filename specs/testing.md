# Testing Specification

## Overview
Monk Manager implements a comprehensive testing strategy that includes unit tests, integration tests, and end-to-end tests, with a focus on reliability and maintainability.

## Test Types

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_parsing() {
        let input = "explain --language rust 'What is a closure?'";
        let command = parse_command(input).unwrap();
        assert_eq!(command.name, "explain");
        assert_eq!(command.language, Some("rust".into()));
    }

    #[test]
    fn test_error_handling() {
        let result = process_command("invalid command");
        assert!(matches!(result, Err(CommandError::InvalidCommand(_))));
    }
}
```

### Integration Tests
```rust
#[tokio::test]
async fn test_explain_command_flow() {
    let config = test_config();
    let app = App::new(config).await.unwrap();
    
    let result = app.execute_command("explain 'What is a closure?'").await;
    assert!(result.is_ok());
    
    let response = result.unwrap();
    assert!(response.contains("closure"));
}
```

### End-to-End Tests
```rust
#[tokio::test]
async fn test_cli_workflow() {
    let output = Command::new("monk-manager")
        .arg("explain")
        .arg("--language")
        .arg("rust")
        .arg("What is a closure?")
        .output()
        .await
        .unwrap();
        
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("closure"));
}
```

## Test Organization

### Test Modules
```rust
// src/commands/explain.rs
#[cfg(test)]
mod tests {
    mod parsing {
        // Command parsing tests
    }
    
    mod execution {
        // Command execution tests
    }
    
    mod error_handling {
        // Error handling tests
    }
}
```

### Test Utilities
```rust
pub mod test_utils {
    pub fn test_config() -> Config {
        Config {
            ai: AIConfig {
                model: "test-model".into(),
                api_key: SecretString::new("test-key".into()),
                temperature: 0.7,
                max_tokens: 1000,
            },
            // ... other fields
        }
    }
    
    pub fn mock_ai_model() -> MockAIModel {
        MockAIModel::new()
    }
}
```

## Mocking

### AI Model Mock
```rust
pub struct MockAIModel {
    responses: HashMap<String, String>,
}

impl MockAIModel {
    pub fn new() -> Self {
        Self {
            responses: HashMap::new(),
        }
    }
    
    pub fn with_response(mut self, prompt: &str, response: &str) -> Self {
        self.responses.insert(prompt.into(), response.into());
        self
    }
}

#[async_trait]
impl AIModel for MockAIModel {
    async fn generate_response(&self, prompt: &str) -> Result<String> {
        self.responses
            .get(prompt)
            .cloned()
            .ok_or_else(|| AIError::ModelError("No response configured".into()))
    }
}
```

### Command Mock
```rust
pub struct MockCommand {
    name: String,
    execute_result: Result<String>,
}

impl MockCommand {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            execute_result: Ok("Mock response".into()),
        }
    }
    
    pub fn with_result(mut self, result: Result<String>) -> Self {
        self.execute_result = result;
        self
    }
}

#[async_trait]
impl Command for MockCommand {
    async fn execute(&self) -> Result<String> {
        self.execute_result.clone()
    }
}
```

## Test Coverage

### Coverage Requirements
- Minimum 80% line coverage
- 100% coverage for critical paths
- All error handling paths tested
- All configuration options tested

### Coverage Reporting
```rust
// .cargo/config.toml
[build]
rustflags = ["-C", "instrument-coverage"]

[profile.test]
opt-level = 0
debug = true
```

## Performance Testing

### Benchmark Tests
```rust
#[bench]
fn bench_command_parsing(b: &mut Bencher) {
    let input = "explain --language rust 'What is a closure?'";
    b.iter(|| parse_command(input));
}

#[bench]
fn bench_ai_response(b: &mut Bencher) {
    let model = test_model();
    let prompt = "What is a closure?";
    b.iter(|| model.generate_response(prompt));
}
```

### Load Testing
```rust
#[tokio::test]
async fn test_concurrent_requests() {
    let app = App::new(test_config()).await.unwrap();
    let mut handles = vec![];
    
    for _ in 0..100 {
        let app = app.clone();
        handles.push(tokio::spawn(async move {
            app.execute_command("explain 'test'").await
        }));
    }
    
    let results = futures::future::join_all(handles).await;
    assert!(results.iter().all(|r| r.is_ok()));
}
```

## Test Documentation

### Test Cases
```rust
/// Test case: Command parsing with valid input
/// 
/// Input: "explain --language rust 'What is a closure?'"
/// Expected: Command parsed successfully with language set to "rust"
#[test]
fn test_command_parsing_valid_input() {
    // Test implementation
}
```

### Test Reports
```rust
#[test]
#[ignore = "Takes too long to run"]
fn test_long_running_operation() {
    // Test implementation
}
```

## Continuous Integration

### GitHub Actions
```yaml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Run tests
        run: cargo test
      - name: Generate coverage
        run: cargo tarpaulin
      - name: Run benchmarks
        run: cargo bench
``` 