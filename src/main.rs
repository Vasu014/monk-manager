use anyhow::Result;
use clap::Parser;
use std::io::{self, Write};
use std::path::PathBuf;
// use tracing::info; // Commented out

mod ai;
mod cli;
mod config;
mod error;
// mod tracing; // Commented out

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: cli::Commands,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    // tracing::init_tracing()?; // Commented out

    // Parse command line arguments
    let cli = Cli::parse();

    // Load configuration
    let mut config = config::Config::load()?;

    // Check and prompt for repository_home if not set
    if config.repository_home.is_none() {
        println!("> No repository home set for this workspace.");
        loop {
            print!("> Please enter the absolute path to your project's root directory: ");
            io::stdout().flush()?; // Ensure the prompt is displayed before input
            let mut repo_path_str = String::new();
            io::stdin().read_line(&mut repo_path_str)?;
            let repo_path_str = repo_path_str.trim();

            if repo_path_str.is_empty() {
                println!("> Path cannot be empty. Please try again.");
                continue;
            }

            let path = PathBuf::from(repo_path_str);
            if !path.is_absolute() {
                println!("> Path must be absolute. Please try again.");
                continue;
            }
            if !path.exists() {
                println!("> Path does not exist: {}. Please try again.", path.display());
                continue;
            }
            if !path.is_dir() {
                println!("> Path is not a directory: {}. Please try again.", path.display());
                continue;
            }

            config.repository_home = Some(repo_path_str.to_string());
            config.save()?; // Config::save is not async
            println!("> Repository home set to: {}", repo_path_str);
            break;
        }
    }

    // Execute command
    match cli.command {
        cli::Commands::Explain(args) => {
            // info!("Executing explain command"); // Commented out
            /*let result =*/ cli::explain::execute(args).await?;
        }
    }

    Ok(())
} 