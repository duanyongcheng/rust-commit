use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use super::{CommitContext, CommitMessage, build_prompt};

pub struct OpenAIClient {
    api_key: String,
    model: String,
    client: reqwest::Client,
}

impl OpenAIClient {
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            api_key,
            model,
            client: reqwest::Client::new(),
        }
    }
    
    pub async fn generate_commit_message(&self, diff: &str, context: &CommitContext) -> Result<CommitMessage> {
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
        
        let response = self.client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await
            .context("Failed to send request to OpenAI")?;
        
        if !response.status().is_success() {
            let error_text = response.text().await?;
            anyhow::bail!("OpenAI API error: {}", error_text);
        }
        
        let api_response: OpenAIResponse = response.json().await
            .context("Failed to parse OpenAI response")?;
        
        let content = api_response.choices
            .first()
            .ok_or_else(|| anyhow::anyhow!("No response from OpenAI"))?
            .message
            .content
            .clone();
        
        let commit_message: CommitMessage = serde_json::from_str(&content)
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
    content: String,
}