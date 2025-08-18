use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use super::{CommitContext, CommitMessage, build_prompt};

pub struct AnthropicClient {
    api_key: String,
    model: String,
    client: reqwest::Client,
}

impl AnthropicClient {
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            api_key,
            model,
            client: reqwest::Client::new(),
        }
    }
    
    pub async fn generate_commit_message(&self, diff: &str, context: &CommitContext) -> Result<CommitMessage> {
        let prompt = build_prompt(diff, context);
        
        let request = AnthropicRequest {
            model: self.model.clone(),
            max_tokens: 500,
            messages: vec![
                AnthropicMessage {
                    role: "user".to_string(),
                    content: format!(
                        "{}\n\nPlease respond with only the JSON object, no other text.",
                        prompt
                    ),
                },
            ],
        };
        
        let response = self.client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to Anthropic")?;
        
        if !response.status().is_success() {
            let error_text = response.text().await?;
            anyhow::bail!("Anthropic API error: {}", error_text);
        }
        
        let api_response: AnthropicResponse = response.json().await
            .context("Failed to parse Anthropic response")?;
        
        let content = api_response.content
            .first()
            .ok_or_else(|| anyhow::anyhow!("No response from Anthropic"))?
            .text
            .clone();
        
        // Extract JSON from the response (Anthropic might include extra text)
        let json_start = content.find('{').unwrap_or(0);
        let json_end = content.rfind('}').map(|i| i + 1).unwrap_or(content.len());
        let json_str = &content[json_start..json_end];
        
        let commit_message: CommitMessage = serde_json::from_str(json_str)
            .context("Failed to parse commit message from Anthropic response")?;
        
        Ok(commit_message)
    }
}

#[derive(Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<AnthropicMessage>,
}

#[derive(Serialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct AnthropicResponse {
    content: Vec<Content>,
}

#[derive(Deserialize)]
struct Content {
    text: String,
}