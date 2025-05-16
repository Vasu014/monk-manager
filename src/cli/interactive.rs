use anyhow::Result;
use std::io::{self, Write};
use std::path::PathBuf;
use crate::ai::{AIService, Message, ModelConfig};
use crate::config::Config;

/// Runs the interactive CLI session.
/// This is the primary interaction mode for monk-manager.
pub async fn run_interactive_session() -> Result<()> {
    // Get the current directory as the project root
    let project_root = std::env::current_dir()?;
    
    // Load configuration
    let config = Config::load()?;
    
    // Initialize AI service
    let ai_service = initialize_ai_service(&config)?;
    
    // Display welcome message with project path
    println!("\x1B[32mWelcome to monk-manager interactive mode!\x1B[0m");
    println!("\x1B[32mProject directory: {}\x1B[0m", project_root.display());
    println!("\x1B[32mType your message and press Enter to send.\x1B[0m");
    println!("\x1B[32mType '/help' for assistance or '/exit' to quit.\x1B[0m\n");

    // Main interaction loop
    let mut conversation_history = Vec::new();
    
    loop {
        print!(">> ");
        io::stdout().flush()?;
        
        // Read user input
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        let input = input.trim();
        
        // Handle empty input
        if input.is_empty() {
            continue;
        }
        
        // Handle special commands
        match input {
            "/exit" | "/quit" => {
                println!("\n\x1B[32mExiting monk-manager.\x1B[0m");
                break;
            },
            "/help" => {
                display_help();
                continue;
            },
            _ => {}
        }
        
        // Add user message to history
        conversation_history.push(Message {
            role: "user".to_string(),
            content: input.to_string(),
        });
        
        // Display "thinking" indicator
        print!("\x1B[33mThinking...\x1B[0m");
        io::stdout().flush()?;
        
        // Get project context
        let project_context = format!("Current directory: {}", project_root.display());
        
        // Get AI response
        match ai_service.chat(&conversation_history, Some(&project_context)).await {
            Ok(response) => {
                // Clear the "thinking" indicator
                print!("\r\x1B[K");
                
                // Display AI response
                println!("\x1B[32m{}\x1B[0m\n", response);
                
                // Add AI response to history
                conversation_history.push(Message {
                    role: "assistant".to_string(),
                    content: response,
                });
            },
            Err(e) => {
                // Clear the "thinking" indicator
                print!("\r\x1B[K");
                
                println!("\x1B[31mError getting AI response: {}\x1B[0m", e);
                println!("\x1B[31mPlease check your API key and internet connection.\x1B[0m");
                println!("\x1B[31mYou can continue chatting, but responses may not work.\x1B[0m\n");
            }
        }
    }
    
    Ok(())
}

// Display help information
fn display_help() {
    println!("\n\x1B[32mAvailable commands:\x1B[0m");
    println!("  \x1B[32m/help\x1B[0m - Display this help message");
    println!("  \x1B[32m/exit\x1B[0m or \x1B[32m/quit\x1B[0m - Exit the session\n");
}

// Initialize the AI service from config
fn initialize_ai_service(_config: &Config) -> Result<AIService> {
    // Use API key from environment variable
    let api_key = std::env::var("ANTHROPIC_API_KEY").unwrap_or_else(|_| {
        println!("\x1B[33mWARNING: ANTHROPIC_API_KEY environment variable not found, using demo key\x1B[0m");
        "demo-api-key".to_string()
    });
    
    if api_key == "demo-api-key" {
        println!("\x1B[31mWARNING: Using demo API key. This won't work for real requests.\x1B[0m");
        println!("\x1B[31mPlease set the ANTHROPIC_API_KEY environment variable to use the service.\x1B[0m");
    }
    
    // Default to Claude model if no configuration exists
    let model_config = ModelConfig {
        provider: "anthropic".to_string(),
        model_name: "claude-3-haiku-20240307".to_string(),
        api_key,
        temperature: 0.7,
        max_tokens: 4000,
        api_base_url: None,
    };
    
    AIService::new(model_config)
} 