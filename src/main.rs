use anyhow::Result;
use clap::Parser;
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
    let config = config::Config::load()?;

    // Execute command
    match cli.command {
        cli::Commands::Explain(args) => {
            // info!("Executing explain command"); // Commented out
            /*let result =*/ cli::explain::execute(args).await?;
        }
    }

    Ok(())
} 