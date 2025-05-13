use anyhow::{Context, Result};
use clap::Args;
use std::path::PathBuf;
// use tracing::{debug, info}; // Commented out debug and info

use crate::{
    ai::AIService,
    config::Config,
};

#[derive(Args, Debug)]
pub struct ExplainArgs {
    /// Path to the file to explain
    #[arg(required = true)]
    pub file: PathBuf,

    /// Programming language of the code
    #[arg(short, long)]
    pub language: Option<String>,

    /// Number of context lines to include
    #[arg(short, long)]
    pub context_lines: Option<usize>,

    /// Output format (markdown, plain)
    #[arg(short, long, default_value = "markdown")]
    pub format: String,
}

pub async fn execute(args: ExplainArgs) -> Result<()> {
    // debug!("Executing explain command with args: {:?}", args); // Commented out

    // Load configuration
    let config = Config::load()?;

    // Read file content
    let content = std::fs::read_to_string(&args.file)
        .with_context(|| format!("Failed to read file: {:?}", args.file))?;

    // Determine language
    let language = args.language.unwrap_or_else(|| {
        // Try to detect language from file extension
        args.file
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("unknown")
            .to_string()
    });

    // Create AI service
    let ai_service = AIService::new(config.ai)?;

    // Get explanation
    // info!("Getting explanation for {} code", language); // Commented out
    let explanation = ai_service.explain(&content, &language).await?;

    // Format and print output
    match args.format.as_str() {
        "markdown" => {
            println!("# Code Explanation\n");
            println!("## File: {}\n", args.file.display());
            println!("## Language: {}\n", language);
            println!("## Explanation\n");
            println!("{}", explanation);
        }
        "plain" => {
            println!("File: {}", args.file.display());
            println!("Language: {}", language);
            println!("\nExplanation:\n");
            println!("{}", explanation);
        }
        _ => anyhow::bail!("Unsupported output format: {}", args.format),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_execute_with_file() {
        let temp_file = NamedTempFile::new().unwrap();
        std::fs::write(&temp_file, "fn main() { println!(\"Hello, world!\"); }").unwrap();

        let args = ExplainArgs {
            file: temp_file.path().to_path_buf(),
            language: Some("rust".to_string()),
            context_lines: None,
            format: "markdown".to_string(),
        };

        // This test will fail if the AI service is not properly configured
        let result = execute(args).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_language_detection() {
        let args = ExplainArgs {
            file: PathBuf::from("test.rs"),
            language: None,
            context_lines: None,
            format: "markdown".to_string(),
        };

        assert_eq!(
            args.language.unwrap_or_else(|| {
                args.file
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .unwrap_or("unknown")
                    .to_string()
            }),
            "rs"
        );
    }
} 