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
    command: Option<cli::Commands>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    // tracing::init_tracing()?; // Commented out

    // Parse command line arguments
    let _cli = Cli::parse();

    // Load configuration
    let _config = config::Config::load()?;

    // Always run in interactive mode
    println!("\x1B[33mNote: monk-manager now always starts in interactive mode\x1B[0m");
    cli::interactive::run_interactive_session().await?;

    Ok(())
} 