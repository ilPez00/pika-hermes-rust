use async_trait::async_trait;
use anyhow::Result;
use crate::agent::llm::client::{LlmClient, LlmResponse, ChatMessage, ToolSpecification, StreamChunk};
use crate::agent::llm::openai::OpenAiClient;
use crate::utils::logger::XLog;
use std::collections::HashMap;
use std::time::Instant;

const TAG: &str = "CascadingLlmClient";

pub struct CascadingLlmClient {
    providers: Vec<Box<dyn LlmClient>>,
    dead_keys: HashMap<String, Instant>,
    dead_duration: std::time::Duration,
}

impl CascadingLlmClient {
    fn key_for(&self, provider: &dyn LlmClient) -> String {
        provider.provider_name().to_string()
    }

    fn is_dead_by_key(&self, key: &str) -> bool {
        self.dead_keys.get(key).map(|until| Instant::now() < *until).unwrap_or(false)
    }

    fn mark_dead(&mut self, key: &str) {
        self.dead_keys.insert(key.to_string(), Instant::now() + self.dead_duration);
    }
}

impl CascadingLlmClient {
    pub fn from_env() -> Self {
        let mut providers: Vec<Box<dyn LlmClient>> = Vec::new();

        if let Ok(url) = std::env::var("OLLAMA_URL") {
            let model = std::env::var("OLLAMA_MODEL").unwrap_or_else(|_| "llama3".into());
            XLog::i(TAG, &format!("Adding Ollama provider: {} ({})", url, model));
            providers.push(Box::new(super::ollama::OllamaClient::new(&url, &model)));
        }

        if let Ok(key) = std::env::var("OPENAI_API_KEY") {
            let url = std::env::var("OPENAI_BASE_URL").unwrap_or_else(|_| "https://api.openai.com/v1".into());
            let model = std::env::var("OPENAI_MODEL").unwrap_or_else(|_| "gpt-4o".into());
            XLog::i(TAG, &format!("Adding OpenAI provider: {} ({})", url, model));
            providers.push(Box::new(OpenAiClient::new(&url, &key, &model)));
        }

        if let Ok(key) = std::env::var("ANTHROPIC_API_KEY") {
            let model = std::env::var("ANTHROPIC_MODEL").unwrap_or_else(|_| "claude-sonnet-4-20250514".into());
            XLog::i(TAG, &format!("Adding Anthropic provider: {}", model));
            // Anthropic uses its own endpoint; for now route through OpenAI-compatible if available
            // or use AnthropicClient when ported
        }

        if let Ok(key) = std::env::var("DEEPSEEK_API_KEY") {
            let model = std::env::var("DEEPSEEK_MODEL").unwrap_or_else(|_| "deepseek-chat".into());
            XLog::i(TAG, &format!("Adding DeepSeek provider: {}", model));
            providers.push(Box::new(OpenAiClient::new("https://api.deepseek.com", &key, &model)));
        }

        if let Ok(key) = std::env::var("GROQ_API_KEY") {
            let model = std::env::var("GROQ_MODEL").unwrap_or_else(|_| "llama3-70b-8192".into());
            XLog::i(TAG, &format!("Adding Groq provider: {}", model));
            providers.push(Box::new(OpenAiClient::new("https://api.groq.com/openai/v1", &key, &model)));
        }

        Self { providers, dead_keys: HashMap::new(), dead_duration: std::time::Duration::from_secs(120) }
    }
}

#[async_trait]
impl LlmClient for CascadingLlmClient {
    fn provider_name(&self) -> &str { "Cascade" }

    async fn chat(&self, messages: &[ChatMessage], tool_specs: &[ToolSpecification]) -> Result<LlmResponse> {
        for provider in &self.providers {
            let key = self.key_for(&**provider);
            if self.is_dead_by_key(&key) {
                XLog::w(TAG, &format!("Skipping dead provider: {}", key));
                continue;
            }
            match provider.chat(messages, tool_specs).await {
                Ok(response) => return Ok(response),
                Err(e) => {
                    XLog::w_err(TAG, &format!("Provider {} failed", key), &e);
                }
            }
        }
        Err(anyhow::anyhow!("All providers failed"))
    }

    async fn chat_streaming(&self, messages: &[ChatMessage], tool_specs: &[ToolSpecification]) -> Result<Vec<StreamChunk>> {
        for provider in &self.providers {
            let key = self.key_for(&**provider);
            if self.is_dead_by_key(&key) {
                continue;
            }
            match provider.chat_streaming(messages, tool_specs).await {
                Ok(chunks) => return Ok(chunks),
                Err(e) => XLog::w_err(TAG, "Provider streaming failed", &e),
            }
        }
        Err(anyhow::anyhow!("All streaming providers failed"))
    }

    fn close(&mut self) {}
}
