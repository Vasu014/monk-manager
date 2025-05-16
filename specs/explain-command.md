# Explain Command Specification

## Overview
The `monkexplain` command provides AI-powered explanations of programming concepts, code snippets, and technical topics.

## Command Syntax
```bash
monk-manager explain [OPTIONS] <INPUT_QUERY_OR_FILE_PATH>
```

## Arguments and Options
*   `<INPUT_QUERY_OR_FILE_PATH>`: (Required) This can be:
    *   A textual query (e.g., "What is a closure in Rust?").
    *   A path to a source code file (e.g., `src/main.rs`). If a file path is provided, it will be resolved relative to the repository home directory (see `specs/cli.md`). The content of the file will be used as the primary input for the explanation.
*   `--language <LANGUAGE>` or `-l <LANGUAGE>`: (Optional) Specifies the programming language of the code snippet or file. If not provided, the CLI may attempt to auto-detect it.
*   `--detail-level <LEVEL>`: (Optional) Specifies the desired level of detail for the explanation (e.g., `basic`, `medium`, `detailed`). Defaults to `medium`.
*   `--format <FORMAT>`: (Optional) Specifies the output format (e.g., `text`, `markdown`, `json`). Defaults to `markdown`.
*   `--target-symbol <SYMBOL_NAME>`: (Optional) If `<INPUT_QUERY_OR_FILE_PATH>` is a file, this option can be used to specify a particular function, struct, class, or variable within that file to focus the explanation on.
*   `--lines <START_LINE>-<END_LINE>`: (Optional) If `<INPUT_QUERY_OR_FILE_PATH>` is a file, this option can specify a range of lines to focus the explanation on (e.g., `10-25`).

## Features

### 1. Input Processing
- Text query handling.
- Source code file reading and parsing (paths relative to repository home).
- Code snippet parsing (if directly provided as part of a text query).
- Language detection
- Context extraction

### 2. Explanation Generation
- Concept explanation
- Code analysis
- Best practices
- Examples

### 3. Output Formatting
- Markdown support
- Syntax highlighting
- Code blocks
- Inline code

## Implementation

### Command Structure
```rust
#[derive(Debug)]
pub struct ExplainCommand {
    input: String,
    language: Option<String>,
    detail_level: DetailLevel,
    format: OutputFormat,
}

#[derive(Debug)]
pub enum DetailLevel {
    Basic,
    Medium,
    Detailed,
}

#[derive(Debug)]
pub enum OutputFormat {
    Text,
    Markdown,
    Json,
}
```

### Command Execution
```rust
impl ExplainCommand {
    #[instrument(skip(self), fields(command = "explain"))]
    pub async fn execute(&self) -> Result<String> {
        info!("Starting explain command");
        
        // Process input
        let processed_input = self.process_input()?;
        
        // Generate explanation
        let explanation = self.generate_explanation(processed_input).await?;
        
        // Format output
        let output = self.format_output(explanation)?;
        
        info!("Explain command completed successfully");
        Ok(output)
    }
}
```

## AI Integration

### Prompt Engineering
```rust
fn generate_prompt(&self, input: &str) -> String {
    format!(
        "Explain the following {} concept in {} detail: {}",
        self.language.as_deref().unwrap_or("programming"),
        self.detail_level.to_string(),
        input
    )
}
```

### Response Processing
```rust
fn process_response(&self, response: &str) -> Result<String> {
    // Process and validate AI response
    // Format according to output format
    // Add syntax highlighting
}
```

## Error Handling

### Error Types
```rust
#[derive(Debug, thiserror::Error)]
pub enum ExplainError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("AI model error: {0}")]
    ModelError(String),
    
    #[error("Formatting error: {0}")]
    FormatError(String),
}
```

## Examples

### Basic Usage
```bash
$ monk-manager explain "What is a closure in Rust?"
```

### Explaining a File
```bash
$ monk-manager explain src/utils/my_module.rs
```

### Explaining a Specific Symbol in a File
```bash
$ monk-manager explain src/models.rs --target-symbol UserProfile
```

### Explaining a Line Range in a File
```bash
$ monk-manager explain src/algorithm.py --lines 50-75
```

### With Options
```bash
$ monk-manager explain --language rust --detail-level detailed "Explain async/await"
```

### Output Example
```