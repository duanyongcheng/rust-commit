use super::{build_prompt, CommitContext, CommitMessage};
use anyhow::{Context, Result};
use colored::*;
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub struct OpenAIClient {
    api_key: String,
    model: String,
    base_url: String,
    client: reqwest::Client,
}

impl OpenAIClient {
    pub fn new(api_key: String, model: String, base_url: Option<String>) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to create HTTP client");
            
        Self {
            api_key,
            model,
            base_url: base_url.unwrap_or_else(|| "https://api.openai.com/v1".to_string()),
            client,
        }
    }

    pub async fn generate_commit_message(
        &self,
        diff: &str,
        context: &CommitContext,
        debug: bool,
    ) -> Result<CommitMessage> {
        let prompt = build_prompt(diff, context);

        let request = OpenAIRequest {
            model: self.model.clone(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: "You are a helpful assistant that generates git commit messages in JSON format.".to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: prompt,
                },
            ],
            temperature: 0.7,
            max_tokens: 500,
            response_format: Some(ResponseFormat {
                type_field: "json_object".to_string(),
            }),
        };

        let response = self
            .client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await
            .context("Failed to send request to OpenAI")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            
            // Sanitize error message to avoid exposing sensitive details
            let safe_error = match status.as_u16() {
                401 => "Authentication failed. Please check your API key.",
                403 => "Access forbidden. Please check your API permissions.",
                429 => "Rate limit exceeded. Please try again later.",
                500..=599 => "OpenAI service error. Please try again later.",
                _ => "Request failed. Please check your configuration.",
            };
            
            if debug {
                eprintln!("Debug: Full error response: {}", error_text);
            }
            
            anyhow::bail!("{} (Status: {})", safe_error, status);
        }

        let response_text = response
            .text()
            .await
            .context("Failed to read response text")?;

        if debug {
            println!("\n{}", "=== DEBUG: Raw HTTP Response ===".cyan().bold());
            println!("{}", response_text);
            println!("{}", "=================================\n".cyan().bold());
        }

        let api_response: OpenAIResponse =
            serde_json::from_str(&response_text).context("Failed to parse OpenAI response")?;

        let content = api_response
            .choices
            .first()
            .ok_or_else(|| anyhow::anyhow!("No response from OpenAI"))?
            .message
            .content
            .clone()
            .ok_or_else(|| anyhow::anyhow!("Response content is null"))?;

        if debug {
            println!("\n{}", "=== DEBUG: AI Message Content ===".cyan().bold());
            println!("{}", content);
            println!("{}", "==================================\n".cyan().bold());
        }

        // Strip markdown code block wrapper if present
        let json_content = if content.starts_with("```json") && content.ends_with("```") {
            content
                .strip_prefix("```json")
                .and_then(|s| s.strip_suffix("```"))
                .map(|s| s.trim())
                .unwrap_or(&content)
        } else if content.starts_with("```") && content.ends_with("```") {
            content
                .strip_prefix("```")
                .and_then(|s| s.strip_suffix("```"))
                .map(|s| s.trim())
                .unwrap_or(&content)
        } else {
            &content
        };

        let commit_message: CommitMessage = serde_json::from_str(json_content)
            .context("Failed to parse commit message from OpenAI response")?;

        Ok(commit_message)
    }
}

#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    max_tokens: u32,
    response_format: Option<ResponseFormat>,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ResponseFormat {
    #[serde(rename = "type")]
    type_field: String,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct ResponseMessage {
    content: Option<String>,
}
