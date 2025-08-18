use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
    
    #[arg(short, long, help = "Path to check (default: current directory)")]
    pub path: Option<PathBuf>,
    
    #[arg(short, long, help = "Verbose output")]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Check repository status (default)
    Status,
    
    /// Generate commit message using AI
    Commit {
        #[arg(long, help = "API key for AI service (or set OPENAI_API_KEY env var)")]
        api_key: Option<String>,
        
        #[arg(long, default_value = "gpt-4", help = "AI model to use")]
        model: String,
        
        #[arg(long, help = "Auto-commit without confirmation")]
        auto: bool,
        
        #[arg(long, help = "Show diff in the prompt")]
        show_diff: bool,
    },
    
    /// Show git diff
    Diff {
        #[arg(long, help = "Show staged changes only")]
        staged: bool,
    },
}