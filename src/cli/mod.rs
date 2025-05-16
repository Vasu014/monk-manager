use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

pub mod explain;

pub use explain::ExplainArgs;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Explain code using AI
    Explain(ExplainArgs),
}

pub async fn execute(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Explain(args) => explain::execute(args).await,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parse() {
        let args = vec!["monk", "explain", "src/main.rs"];
        let cli = Cli::parse_from(args);
        assert!(matches!(cli.command, Commands::Explain(_)));
    }

    #[test]
    fn test_cli_parse_with_language() {
        let args = vec!["monk", "explain", "src/main.rs", "--language", "rust"];
        let cli = Cli::parse_from(args);
        if let Commands::Explain(args) = cli.command {
            assert_eq!(args.language, Some("rust".to_string()));
        } else {
            panic!("Expected Explain command");
        }
    }
} 