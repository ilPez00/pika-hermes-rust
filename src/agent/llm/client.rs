use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmResponse {
    pub text: Option<String>,
    pub tool_execution_requests: Vec<ToolExecutionRequest>,
    pub model_name: Option<String>,
    pub token_usage: Option<TokenUsage>,
    pub has_tool_execution_requests: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolExecutionRequest {
    pub id: String,
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    pub input_tokens: Option<i32>,
    pub output_tokens: Option<i32>,
    pub total_tokens: Option<i32>,
}

impl TokenUsage {
    pub fn total_token_count(&self) -> i32 {
        self.total_tokens.unwrap_or(0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamChunk {
    pub token: String,
    pub is_tool_call: bool,
}

#[async_trait]
pub trait LlmClient: Send + Sync {
    fn provider_name(&self) -> &str;
    async fn chat(&self, messages: &[ChatMessage], tool_specs: &[ToolSpecification]) -> Result<LlmResponse>;
    async fn chat_streaming(&self, messages: &[ChatMessage], tool_specs: &[ToolSpecification]) -> Result<Vec<StreamChunk>>;
    fn close(&mut self);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
    pub tool_call_id: Option<String>,
    pub tool_name: Option<String>,
}

impl ChatMessage {
    pub fn system(text: &str) -> Self {
        Self { role: "system".into(), content: text.into(), tool_call_id: None, tool_name: None }
    }
    pub fn user(text: &str) -> Self {
        Self { role: "user".into(), content: text.into(), tool_call_id: None, tool_name: None }
    }
    pub fn assistant(text: &str) -> Self {
        Self { role: "assistant".into(), content: text.into(), tool_call_id: None, tool_name: None }
    }
    pub fn tool_result(tool_call_id: &str, tool_name: &str, result: &str) -> Self {
        Self { role: "tool".into(), content: result.into(), tool_call_id: Some(tool_call_id.into()), tool_name: Some(tool_name.into()) }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolSpecification {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

#[derive(Clone)]
pub enum LlmProvider {
    Local,
    OpenAi,
    Ollama,
    Cascade,
    Mock,
}

impl LlmProvider {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "local" | "llama" => LlmProvider::Local,
            "ollama" => LlmProvider::Ollama,
            "cascade" => LlmProvider::Cascade,
            "mock" => LlmProvider::Mock,
            _ => LlmProvider::OpenAi,
        }
    }
}
