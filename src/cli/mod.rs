use anyhow::Result;
use clap::{Parser, Subcommand};

pub mod explain;
pub mod interactive;

pub use explain::ExplainArgs;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// [Legacy] Explain code using AI - now redirects to interactive mode
    Explain(ExplainArgs),
}

// This function is no longer used since we always start interactive mode
#[deprecated(note = "All commands now redirect to interactive mode")]
pub async fn execute(cli: Cli) -> Result<()> {
    // Always use interactive mode now
    interactive::run_interactive_session().await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parse() {
        let args = vec!["monk", "explain", "src/main.rs"];
        let cli = Cli::parse_from(args);
        assert!(matches!(cli.command, Some(Commands::Explain(_))));
    }

    #[test]
    fn test_cli_parse_with_language() {
        let args = vec!["monk", "explain", "src/main.rs", "--language", "rust"];
        let cli = Cli::parse_from(args);
        if let Some(Commands::Explain(args)) = cli.command {
            assert_eq!(args.language, Some("rust".to_string()));
        } else {
            panic!("Expected Explain command");
        }
    }

    #[test]
    fn test_cli_no_command() {
        let args = vec!["monk"];
        let cli = Cli::parse_from(args);
        assert!(matches!(cli.command, None));
    }
} 