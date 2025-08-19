mod ai;
mod cli;
mod config;
mod git;
mod ui;

use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use std::env;
use std::process::Command;

use crate::cli::{Args, Commands};
use crate::config::Config;
use crate::git::GitRepo;
use crate::ui::{CommitAction, CommitUI};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let path = args.path.unwrap_or_else(|| env::current_dir().unwrap());

    // Handle init command first (doesn't need git repo)
    if let Some(Commands::Init { local, force }) = &args.command {
        return handle_init_command(*local, *force);
    }

    // Check if it's a git repository
    let repo = match GitRepo::open(&path) {
        Ok(repo) => repo,
        Err(_) => {
            println!("{} {}", "Checking:".bold(), path.display());
            println!();
            println!("{} ✗", "Not a Git repository".red().bold());
            println!();
            println!(
                "{}",
                "Tip: Run 'git init' to initialize a Git repository".yellow()
            );
            return Ok(());
        }
    };

    // Handle commands
    match args.command {
        Some(Commands::Commit {
            api_key,
            model,
            base_url,
            auto,
            show_diff,
            debug,
        }) => {
            handle_commit_command(repo, api_key, model, base_url, auto, show_diff, debug).await?;
        }
        Some(Commands::Diff { staged }) => {
            handle_diff_command(repo, staged)?;
        }
        Some(Commands::Status) | None => {
            handle_status_command(repo, args.verbose)?;
        }
        Some(Commands::Init { .. }) => {
            // Already handled above
            unreachable!()
        }
    }

    Ok(())
}

fn handle_init_command(local: bool, force: bool) -> Result<()> {
    match Config::init(local, force) {
        Ok(path) => {
            println!(
                "{} Configuration file created at: {}",
                "✓".green().bold(),
                path.display()
            );
            println!();
            println!("{}", "Next steps:".bold());
            println!("  1. Edit the config file to set your API provider and model");
            println!("  2. Set your API key either:");
            println!("     - In the environment variable (recommended)");
            println!("     - Directly in the config file (not recommended)");
            println!();
            println!("  Example:");
            println!("  export OPENAI_API_KEY=\"your-api-key\"");
            println!("  # or");
            println!("  export ANTHROPIC_API_KEY=\"your-api-key\"");
            Ok(())
        }
        Err(e) => {
            eprintln!("{} {}", "Error:".red().bold(), e);
            Err(e)
        }
    }
}

fn handle_status_command(repo: GitRepo, verbose: bool) -> Result<()> {
    println!("{} {}", "Checking:".bold(), env::current_dir()?.display());
    println!();
    println!("{} ✓", "Git repository detected".green().bold());

    let status = repo.get_status()?;

    if status.is_clean {
        println!("{} ✓", "Working tree clean".green().bold());
        println!("All changes have been committed.");
    } else {
        println!("{} ✗", "Uncommitted changes detected".yellow().bold());
        println!();

        if !status.modified_files.is_empty() {
            println!("{}:", "Modified files".yellow());
            for file in &status.modified_files {
                println!("  {} {}", "M".yellow(), file);
            }
            println!();
        }

        if !status.new_files.is_empty() {
            println!("{}:", "New files".green());
            for file in &status.new_files {
                println!("  {} {}", "A".green(), file);
            }
            println!();
        }

        if !status.deleted_files.is_empty() {
            println!("{}:", "Deleted files".red());
            for file in &status.deleted_files {
                println!("  {} {}", "D".red(), file);
            }
            println!();
        }

        if !status.renamed_files.is_empty() {
            println!("{}:", "Renamed files".blue());
            for file in &status.renamed_files {
                println!("  {} {}", "R".blue(), file);
            }
            println!();
        }

        println!(
            "{}: {}",
            "Total uncommitted changes".bold(),
            status.total_changes().to_string().yellow()
        );

        if verbose {
            println!();
            println!("{}", "Tip: Use 'git add .' to stage all changes".cyan());
            println!(
                "{}",
                "     Use 'git commit -m \"message\"' to commit staged changes".cyan()
            );
            println!(
                "{}",
                "     Use 'rust-commit commit' to generate AI commit message".cyan()
            );
        }
    }

    // Show branch info
    let branch_info = repo.get_branch_info()?;
    println!();

    if let Some(name) = branch_info.name {
        if name == "unborn" {
            println!(
                "{}: No commits yet (unborn branch)",
                "Branch".yellow().bold()
            );
        } else {
            println!("{}: {}", "Current branch".bold(), name.cyan());

            if let Some(tracking) = branch_info.tracking_info {
                println!("{}: {}", "Tracking".bold(), tracking.upstream.cyan());

                if tracking.ahead > 0 || tracking.behind > 0 {
                    let mut status_parts = Vec::new();
                    if tracking.ahead > 0 {
                        status_parts.push(format!("{} ahead", tracking.ahead).green().to_string());
                    }
                    if tracking.behind > 0 {
                        status_parts
                            .push(format!("{} behind", tracking.behind).yellow().to_string());
                    }
                    println!("{}: {}", "Status".bold(), status_parts.join(", "));
                }
            }
        }
    } else if branch_info.is_detached {
        println!("{}: detached", "HEAD state".yellow().bold());
    }

    Ok(())
}

fn handle_diff_command(repo: GitRepo, staged: bool) -> Result<()> {
    let diff = if staged {
        println!("{}", "Showing staged changes:".bold().green());
        repo.get_diff(true)?
    } else {
        println!("{}", "Showing all changes:".bold().green());
        repo.get_combined_diff()?
    };

    if diff.is_empty() {
        println!("{}", "No changes to show".yellow());
    } else {
        println!("{}", diff);
    }

    Ok(())
}

async fn handle_commit_command(
    repo: GitRepo,
    api_key: Option<String>,
    model: Option<String>,
    base_url: Option<String>,
    auto: bool,
    show_diff: bool,
    debug: bool,
) -> Result<()> {
    // Load config
    let config = Config::load().unwrap_or_default();

    // Check for changes
    let status = repo.get_status()?;
    if status.is_clean {
        CommitUI::show_info("No changes to commit");
        return Ok(());
    }

    // Get diff
    let diff = repo.get_combined_diff()?;
    if diff.is_empty() {
        CommitUI::show_info("No changes detected");
        return Ok(());
    }

    // Show diff preview if requested
    if show_diff && !CommitUI::show_diff_preview(&diff, 30)? {
        CommitUI::show_info("Commit generation cancelled");
        return Ok(());
    }

    // Get API key
    let api_key = api_key
        .or_else(|| config.get_api_key())
        .or_else(|| CommitUI::get_api_key(&config.ai.provider).ok())
        .context("No API key provided")?;

    // Count changes for context
    let added_lines = diff.lines().filter(|l| l.starts_with('+')).count();
    let removed_lines = diff.lines().filter(|l| l.starts_with('-')).count();

    // Get branch info
    let branch_info = repo.get_branch_info()?;

    // Create context
    let context = ai::CommitContext {
        branch_name: branch_info.name,
        file_count: status.total_changes(),
        added_lines,
        removed_lines,
    };

    // Create AI client
    // Use model from CLI if provided, otherwise use config
    let final_model = model.unwrap_or(config.ai.model.clone());
    // Use base_url from CLI if provided, otherwise use config
    let final_base_url = base_url.or(config.ai.base_url.clone());
    let client = ai::create_client(&config.ai.provider, api_key, final_model, final_base_url)?;

    CommitUI::show_info("Generating commit message with AI...");

    // Generate commit message
    let commit_message = client
        .generate_commit_message(&diff, &context, debug)
        .await?;

    // Handle user action
    let action = if auto {
        CommitAction::Accept
    } else {
        CommitUI::confirm_commit(&commit_message)?
    };

    match action {
        CommitAction::Accept => {
            execute_commit(&commit_message.format_conventional())?;
            CommitUI::show_success("Changes committed successfully!");
        }
        CommitAction::Edit(edited_message) => {
            execute_commit(&edited_message)?;
            CommitUI::show_success("Changes committed with edited message!");
        }
        CommitAction::Regenerate => {
            CommitUI::show_info("Please run the command again to regenerate");
        }
        CommitAction::Cancel => {
            CommitUI::show_info("Commit cancelled");
        }
    }

    Ok(())
}

fn execute_commit(message: &str) -> Result<()> {
    // First, stage all changes
    Command::new("git")
        .args(["add", "."])
        .output()
        .context("Failed to stage changes")?;

    // Then commit with the message
    let output = Command::new("git")
        .args(["commit", "-m", message])
        .output()
        .context("Failed to execute git commit")?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Git commit failed: {}", error);
    }

    Ok(())
}
