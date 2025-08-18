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