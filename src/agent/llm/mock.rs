use async_trait::async_trait;
use anyhow::Result;
use crate::agent::llm::client::{LlmClient, LlmResponse, ChatMessage, ToolSpecification, StreamChunk, ToolExecutionRequest};
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct MockLlmClient {
    call_count: AtomicUsize,
}

impl MockLlmClient {
    pub fn new() -> Self {
        Self { call_count: AtomicUsize::new(0) }
    }
}

#[async_trait]
impl LlmClient for MockLlmClient {
    fn provider_name(&self) -> &str { "Mock" }

    async fn chat(&self, _messages: &[ChatMessage], _tool_specs: &[ToolSpecification]) -> Result<LlmResponse> {
        let count = self.call_count.fetch_add(1, Ordering::SeqCst);

        // First call: return a tool call to get_device_info
        if count == 0 {
            return Ok(LlmResponse {
                text: None,
                tool_execution_requests: vec![ToolExecutionRequest {
                    id: "call_1".to_string(),
                    name: "get_device_info".to_string(),
                    arguments: r#"{"category": "time"}"#.to_string(),
                }],
                model_name: Some("mock".to_string()),
                token_usage: None,
                has_tool_execution_requests: true,
            });
        }

        // Subsequent: return final answer
        Ok(LlmResponse {
            text: Some("Final answer: The current time is 2026-05-31 16:30:00 UTC+2.".to_string()),
            tool_execution_requests: Vec::new(),
            model_name: Some("mock".to_string()),
            token_usage: None,
            has_tool_execution_requests: false,
        })
    }

    async fn chat_streaming(&self, _messages: &[ChatMessage], _tool_specs: &[ToolSpecification]) -> Result<Vec<StreamChunk>> {
        Ok(Vec::new())
    }

    fn close(&mut self) {}
}
