use async_trait::async_trait;
use anyhow::Result;
use crate::agent::llm::client::{LlmClient, LlmResponse, ChatMessage, ToolSpecification, StreamChunk, TokenUsage, ToolExecutionRequest};
use crate::utils::logger::XLog;

const TAG: &str = "OpenAiClient";

pub struct OpenAiClient {
    base_url: String,
    api_key: String,
    model: String,
    client: reqwest::Client,
}

impl OpenAiClient {
    pub fn new(base_url: &str, api_key: &str, model: &str) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            api_key: api_key.to_string(),
            model: model.to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub fn from_env() -> Option<Self> {
        let api_key = std::env::var("OPENAI_API_KEY").ok()?;
        let base_url = std::env::var("OPENAI_BASE_URL").unwrap_or_else(|_| "https://api.openai.com/v1".into());
        let model = std::env::var("OPENAI_MODEL").unwrap_or_else(|_| "gpt-4o".into());
        Some(Self::new(&base_url, &api_key, &model))
    }

    fn messages_to_json(&self, messages: &[ChatMessage]) -> Vec<serde_json::Value> {
        messages.iter().map(|m| {
            let mut obj = serde_json::json!({
                "role": m.role,
                "content": m.content,
            });
            if let Some(ref id) = m.tool_call_id {
                obj["tool_call_id"] = serde_json::Value::String(id.clone());
            }
            obj
        }).collect()
    }

    fn tools_to_json(&self, tool_specs: &[ToolSpecification]) -> Vec<serde_json::Value> {
        tool_specs.iter().map(|t| {
            serde_json::json!({
                "type": "function",
                "function": {
                    "name": t.name,
                    "description": t.description,
                    "parameters": t.parameters,
                }
            })
        }).collect()
    }

    fn parse_tool_calls(&self, data: &serde_json::Value) -> Vec<ToolExecutionRequest> {
        let mut requests = Vec::new();
        if let Some(choices) = data["choices"].as_array() {
            if let Some(choice) = choices.first() {
                if let Some(tool_calls) = choice["message"]["tool_calls"].as_array() {
                    for tc in tool_calls {
                        if let (Some(name), Some(arguments)) = (tc["function"]["name"].as_str(), tc["function"]["arguments"].as_str()) {
                            requests.push(ToolExecutionRequest {
                                id: tc["id"].as_str().unwrap_or("call_unknown").to_string(),
                                name: name.to_string(),
                                arguments: arguments.to_string(),
                            });
                        }
                    }
                }
            }
        }
        requests
    }
}

#[async_trait]
impl LlmClient for OpenAiClient {
    fn provider_name(&self) -> &str { "OpenAI" }

    async fn chat(&self, messages: &[ChatMessage], tool_specs: &[ToolSpecification]) -> Result<LlmResponse> {
        let mut body = serde_json::json!({
            "model": self.model,
            "messages": self.messages_to_json(messages),
        });

        if !tool_specs.is_empty() {
            body["tools"] = serde_json::Value::Array(self.tools_to_json(tool_specs));
            body["tool_choice"] = serde_json::json!("auto");
        }

        XLog::d(TAG, &format!("POST {}/chat/completions with {} tools", self.base_url, tool_specs.len()));

        let resp = self.client.post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!("OpenAI API error {}: {}", status, text));
        }

        let data: serde_json::Value = resp.json().await?;

        let text = data["choices"][0]["message"]["content"].as_str().map(|s| s.to_string());
        let tool_calls = self.parse_tool_calls(&data);

        let input_tokens = data["usage"]["prompt_tokens"].as_i64().map(|n| n as i32);
        let output_tokens = data["usage"]["completion_tokens"].as_i64().map(|n| n as i32);

        Ok(LlmResponse {
            text,
            tool_execution_requests: tool_calls.clone(),
            model_name: Some(self.model.clone()),
            token_usage: Some(TokenUsage {
                input_tokens,
                output_tokens,
                total_tokens: Some(input_tokens.unwrap_or(0) + output_tokens.unwrap_or(0)),
            }),
            has_tool_execution_requests: !tool_calls.is_empty(),
        })
    }

    async fn chat_streaming(&self, messages: &[ChatMessage], tool_specs: &[ToolSpecification]) -> Result<Vec<StreamChunk>> {
        let mut body = serde_json::json!({
            "model": self.model,
            "messages": self.messages_to_json(messages),
            "stream": true,
        });

        if !tool_specs.is_empty() {
            body["tools"] = serde_json::Value::Array(self.tools_to_json(tool_specs));
        }

        let resp = self.client.post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        if !resp.status().is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!("Streaming API error: {}", text));
        }

        let mut chunks = Vec::new();
        let bytes = resp.bytes().await?;
        let text = String::from_utf8_lossy(&bytes);
        chunks.push(StreamChunk { token: text.to_string(), is_tool_call: false });
        Ok(chunks)
    }

    fn close(&mut self) {}
}
