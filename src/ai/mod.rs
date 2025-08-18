use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

pub mod openai;
pub mod anthropic;

#[derive(Debug, Clone)]
pub struct CommitContext {
    pub branch_name: Option<String>,
    pub file_count: usize,
    pub added_lines: usize,
    pub removed_lines: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommitMessage {
    pub commit_type: String,
    pub scope: Option<String>,
    pub description: String,
    pub body: Option<String>,
    pub breaking_change: Option<String>,
}

impl CommitMessage {
    pub fn format_conventional(&self) -> String {
        let mut message = String::new();
        
        // Header: type(scope): description
        message.push_str(&self.commit_type);
        if let Some(scope) = &self.scope {
            message.push_str(&format!("({})", scope));
        }
        message.push_str(": ");
        message.push_str(&self.description);
        
        // Body
        if let Some(body) = &self.body {
            message.push_str("\n\n");
            message.push_str(body);
        }
        
        // Breaking change
        if let Some(breaking) = &self.breaking_change {
            message.push_str("\n\n");
            message.push_str("BREAKING CHANGE: ");
            message.push_str(breaking);
        }
        
        message
    }
    
    pub fn format_simple(&self) -> String {
        self.description.clone()
    }
}

pub enum AIClient {
    OpenAI(openai::OpenAIClient),
    Anthropic(anthropic::AnthropicClient),
}

impl AIClient {
    pub async fn generate_commit_message(&self, diff: &str, context: &CommitContext) -> Result<CommitMessage> {
        match self {
            AIClient::OpenAI(client) => client.generate_commit_message(diff, context).await,
            AIClient::Anthropic(client) => client.generate_commit_message(diff, context).await,
        }
    }
}

pub fn create_client(provider: &str, api_key: String, model: String) -> Result<AIClient> {
    match provider.to_lowercase().as_str() {
        "openai" => Ok(AIClient::OpenAI(openai::OpenAIClient::new(api_key, model))),
        "anthropic" => Ok(AIClient::Anthropic(anthropic::AnthropicClient::new(api_key, model))),
        _ => anyhow::bail!("Unsupported AI provider: {}", provider),
    }
}

pub fn build_prompt(diff: &str, context: &CommitContext) -> String {
    format!(
        r#"You are a Git commit message generator. Based on the following git diff, generate a structured commit message.

Context:
- Branch: {}
- Files changed: {}
- Lines added: {}
- Lines removed: {}

Git Diff:
```
{}
```

Generate a commit message following the Conventional Commits specification:
- type: feat, fix, docs, style, refactor, test, chore, perf
- scope: optional, the component or area affected
- description: brief description (50 chars or less)
- body: optional, detailed explanation
- breaking_change: optional, if there are breaking changes

Respond with a JSON object containing these fields.
"#,
        context.branch_name.as_deref().unwrap_or("unknown"),
        context.file_count,
        context.added_lines,
        context.removed_lines,
        truncate_diff(diff, 3000)
    )
}

fn truncate_diff(diff: &str, max_chars: usize) -> &str {
    if diff.len() <= max_chars {
        diff
    } else {
        &diff[..max_chars]
    }
}