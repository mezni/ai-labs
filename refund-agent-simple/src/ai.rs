use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: MessageResp,
}

#[derive(Deserialize)]
struct MessageResp {
    content: String,
}

pub async fn review_refund(order_id: &str, amount: f64, reason: &str) -> Result<String> {
    let api_key = env::var("OPENAI_API_KEY").context("OPENAI_API_KEY not set")?;
    let client = Client::new();

    let prompt = format!(
        "You are a refund agent reviewer. Review this refund request and give a short recommendation (approve/reject) with brief reason.\n\nOrder: {}\nAmount: {:.2}\nReason: {}\n\nRespond in 1-2 sentences.",
        order_id, amount, reason
    );

    let req = ChatRequest {
        model: "gpt-6-luna".to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: prompt,
        }],
        temperature: 0.2,
    };

    let resp: ChatResponse = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&req)
        .send()
        .await?
        .json()
        .await?;

    let review = resp
        .choices
        .first()
        .map(|c| c.message.content.trim().to_string())
        .unwrap_or_else(|| "No review available".to_string());

    Ok(review)
}
