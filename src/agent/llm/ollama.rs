use async_trait::async_trait;
use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::agent::llm::client::{LlmClient, LlmResponse, ChatMessage, ToolSpecification, StreamChunk};

const TAG: &str = "OllamaClient";

#[derive(Serialize)]
struct OllamaChatRequest {
    model: String,
    messages: Vec<OllamaMessage>,
    stream: bool,
}

#[derive(Serialize)]
struct OllamaMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OllamaChatResponse {
    message: OllamaResponseMessage,
    model: String,
}

#[derive(Deserialize)]
struct OllamaResponseMessage {
    content: String,
}

#[derive(Serialize)]
struct OllamaGenerateRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaGenerateResponse {
    response: String,
}

pub struct OllamaClient {
    client: Client,
    url: String,
    model: String,
}

impl OllamaClient {
    pub fn new(url: &str, model: &str) -> Self {
        let url = url.trim_end_matches('/').to_string();
        Self {
            client: Client::new(),
            url,
            model: model.to_string(),
        }
    }
}

#[async_trait]
impl LlmClient for OllamaClient {
    fn provider_name(&self) -> &str { "Ollama" }

    async fn chat(&self, messages: &[ChatMessage], _tool_specs: &[ToolSpecification]) -> Result<LlmResponse> {
        let ollama_msgs: Vec<OllamaMessage> = messages.iter().map(|m| {
            OllamaMessage {
                role: m.role.clone(),
                content: m.content.clone(),
            }
        }).collect();

        let req = if ollama_msgs.len() == 1 && ollama_msgs[0].role == "user" {
            OllamaGenerateRequest {
                model: self.model.clone(),
                prompt: ollama_msgs[0].content.clone(),
                stream: false,
            }
        } else {
            let req = OllamaChatRequest {
                model: self.model.clone(),
                messages: ollama_msgs,
                stream: false,
            };
            let res = self.client.post(format!("{}/api/chat", self.url))
                .json(&req)
                .send()
                .await?;

            if !res.status().is_success() {
                return Err(anyhow::anyhow!("Ollama chat error: {}", res.status()));
            }

            let data: OllamaChatResponse = res.json().await?;
            return Ok(LlmResponse {
                text: Some(data.message.content),
                tool_execution_requests: Vec::new(),
                model_name: Some(data.model),
                token_usage: None,
                has_tool_execution_requests: false,
            });
        };

        let res = self.client.post(format!("{}/api/generate", self.url))
            .json(&req)
            .send()
            .await?;

        if !res.status().is_success() {
            return Err(anyhow::anyhow!("Ollama generate error: {}", res.status()));
        }

        let data: OllamaGenerateResponse = res.json().await?;
        Ok(LlmResponse {
            text: Some(data.response),
            tool_execution_requests: Vec::new(),
            model_name: Some(self.model.clone()),
            token_usage: None,
            has_tool_execution_requests: false,
        })
    }

    async fn chat_streaming(&self, messages: &[ChatMessage], _tool_specs: &[ToolSpecification]) -> Result<Vec<StreamChunk>> {
        let ollama_msgs: Vec<OllamaMessage> = messages.iter().map(|m| {
            OllamaMessage { role: m.role.clone(), content: m.content.clone() }
        }).collect();

        let req = OllamaChatRequest {
            model: self.model.clone(),
            messages: ollama_msgs,
            stream: true,
        };

        let response = self.client.post(format!("{}/api/chat", self.url))
            .json(&req)
            .send()
            .await?;

        let mut chunks = Vec::new();
        let body = response.bytes().await?;
        let text = String::from_utf8_lossy(&body);
        chunks.push(StreamChunk { token: text.to_string(), is_tool_call: false });
        Ok(chunks)
    }

    fn close(&mut self) {}
}
