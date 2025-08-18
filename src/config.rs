use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub ai: AIConfig,
    pub commit: CommitConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AIConfig {
    pub provider: String,
    pub model: String,
    pub api_key_env: String,
    pub api_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommitConfig {
    pub format: String,
    pub include_emoji: bool,
    pub max_diff_size: usize,
    pub auto_stage: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ai: AIConfig {
                provider: "openai".to_string(),
                model: "gpt-4".to_string(),
                api_key_env: "OPENAI_API_KEY".to_string(),
                api_key: None,
            },
            commit: CommitConfig {
                format: "conventional".to_string(),
                include_emoji: false,
                max_diff_size: 4000,
                auto_stage: false,
            },
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        // Try to load from multiple locations
        let config_paths = vec![
            PathBuf::from(".rust-commit.toml"),
            dirs::config_dir()
                .map(|p| p.join("rust-commit/config.toml"))
                .unwrap_or_default(),
            dirs::home_dir()
                .map(|p| p.join(".rust-commit.toml"))
                .unwrap_or_default(),
        ];
        
        for path in config_paths {
            if path.exists() {
                let content = fs::read_to_string(&path)
                    .context(format!("Failed to read config from {:?}", path))?;
                let config: Config = toml::from_str(&content)
                    .context(format!("Failed to parse config from {:?}", path))?;
                return Ok(config);
            }
        }
        
        // Return default if no config file found
        Ok(Self::default())
    }
    
    pub fn save(&self, path: Option<PathBuf>) -> Result<()> {
        let path = path.unwrap_or_else(|| PathBuf::from(".rust-commit.toml"));
        
        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .context(format!("Failed to create directory {:?}", parent))?;
            }
        }
        
        let content = toml::to_string_pretty(self)
            .context("Failed to serialize config")?;
        fs::write(&path, content)
            .context(format!("Failed to write config to {:?}", path))?;
        Ok(())
    }
    
    pub fn get_api_key(&self) -> Option<String> {
        // First check if api_key is directly set
        if let Some(key) = &self.ai.api_key {
            return Some(key.clone());
        }
        
        // Then check environment variable
        std::env::var(&self.ai.api_key_env).ok()
    }
    
    pub fn init(local: bool, force: bool) -> Result<PathBuf> {
        let path = if local {
            PathBuf::from(".rust-commit.toml")
        } else {
            // Try to use ~/.config/rust-commit/config.toml first
            dirs::config_dir()
                .map(|p| p.join("rust-commit/config.toml"))
                .or_else(|| dirs::home_dir().map(|p| p.join(".rust-commit.toml")))
                .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?
        };
        
        // Check if file already exists
        if path.exists() && !force {
            anyhow::bail!(
                "Config file already exists at {:?}. Use --force to overwrite.",
                path
            );
        }
        
        // Create default config with helpful comments
        let config_content = r#"# Rust Commit Configuration File
# This file configures the rust-commit tool for AI-powered commit message generation

[ai]
# AI provider: "openai" or "anthropic"
provider = "openai"

# Model to use for generation
# OpenAI: "gpt-4", "gpt-4-turbo", "gpt-3.5-turbo"
# Anthropic: "claude-3-opus", "claude-3-sonnet", "claude-3-haiku"
model = "gpt-4"

# Environment variable containing the API key
# For OpenAI: typically "OPENAI_API_KEY"
# For Anthropic: typically "ANTHROPIC_API_KEY"
api_key_env = "OPENAI_API_KEY"

# Direct API key (not recommended for security reasons)
# Uncomment and set your API key here if you prefer not to use environment variables
# api_key = "your-api-key-here"

[commit]
# Commit message format: "conventional" (follows Conventional Commits spec)
format = "conventional"

# Whether to include emoji in commit messages
include_emoji = false

# Maximum diff size in characters to send to AI
max_diff_size = 4000

# Whether to automatically stage all changes before committing
auto_stage = false
"#;
        
        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .context(format!("Failed to create directory {:?}", parent))?;
            }
        }
        
        // Write config file
        fs::write(&path, config_content)
            .context(format!("Failed to write config to {:?}", path))?;
        
        Ok(path)
    }
    
    pub fn create_example() -> Result<()> {
        let example = Config::default();
        let content = toml::to_string_pretty(&example)?;
        let example_content = format!(
            "# Rust Commit Configuration File\n\
             # Place this file in your project root or home directory\n\n\
             {}\n\n\
             # You can also set the API key directly (not recommended):\n\
             # [ai]\n\
             # api_key = \"your-api-key-here\"\n",
            content
        );
        
        fs::write(".rust-commit.example.toml", example_content)
            .context("Failed to create example config")?;
        
        Ok(())
    }
}