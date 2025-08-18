use anyhow::Result;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm, Editor, Select};
use crate::ai::CommitMessage;

pub struct CommitUI;

impl CommitUI {
    pub fn confirm_commit(message: &CommitMessage) -> Result<CommitAction> {
        println!("\n{}", "Generated Commit Message:".bold().green());
        println!("{}", "─".repeat(50));
        
        // Display formatted message
        println!("{}", message.format_conventional().cyan());
        println!("{}", "─".repeat(50));
        
        // Show options
        let options = vec![
            "Accept and commit",
            "Edit message",
            "Regenerate with different AI",
            "Cancel",
        ];
        
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What would you like to do?")
            .items(&options)
            .default(0)
            .interact()?;
        
        match selection {
            0 => Ok(CommitAction::Accept),
            1 => {
                let edited = Editor::new()
                    .edit(&message.format_conventional())?
                    .unwrap_or_else(|| message.format_conventional());
                Ok(CommitAction::Edit(edited))
            }
            2 => Ok(CommitAction::Regenerate),
            _ => Ok(CommitAction::Cancel),
        }
    }
    
    pub fn show_diff_preview(diff: &str, max_lines: usize) -> Result<bool> {
        let lines: Vec<&str> = diff.lines().collect();
        let total_lines = lines.len();
        
        println!("\n{}", "Diff Preview:".bold().yellow());
        println!("{}", "─".repeat(50));
        
        for (_i, line) in lines.iter().take(max_lines).enumerate() {
            let colored_line = if line.starts_with('+') && !line.starts_with("+++") {
                line.green()
            } else if line.starts_with('-') && !line.starts_with("---") {
                line.red()
            } else if line.starts_with("@@") {
                line.cyan()
            } else {
                line.normal()
            };
            println!("{}", colored_line);
        }
        
        if total_lines > max_lines {
            println!("\n... {} more lines ...", total_lines - max_lines);
        }
        
        println!("{}", "─".repeat(50));
        
        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Continue with commit generation?")
            .default(true)
            .interact()
            .map_err(Into::into)
    }
    
    pub fn get_api_key(provider: &str) -> Result<String> {
        println!("\n{}", format!("{} API key not found!", provider).yellow());
        println!("Please enter your {} API key:", provider);
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_string())
    }
    
    pub fn show_success(message: &str) {
        println!("{} {}", "✓".green().bold(), message.green());
    }
    
    pub fn show_info(message: &str) {
        println!("{} {}", "ℹ".blue(), message);
    }
}

pub enum CommitAction {
    Accept,
    Edit(String),
    Regenerate,
    Cancel,
}