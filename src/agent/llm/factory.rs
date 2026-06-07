use crate::agent::llm::client::{LlmClient, LlmProvider};
use crate::agent::llm::cascade::CascadingLlmClient;
use crate::agent::llm::ollama::OllamaClient;
use crate::agent::llm::openai::OpenAiClient;
use crate::agent::llm::mock::MockLlmClient;
use crate::utils::logger::XLog;
use crate::agent::config::AgentConfig;

const TAG: &str = "LlmClientFactory";

pub struct LlmClientFactory;

impl LlmClientFactory {
    pub fn create(config: &AgentConfig) -> Box<dyn LlmClient> {
        match config.provider {
            LlmProvider::Mock => {
                XLog::i(TAG, "Creating MockLlmClient for testing");
                Box::new(MockLlmClient::new())
            }
            LlmProvider::Ollama => {
                let url = std::env::var("OLLAMA_URL").unwrap_or_else(|_| "http://localhost:11434".into());
                let model = config.model_name.clone().unwrap_or_else(|| "llama3".into());
                XLog::i(TAG, &format!("Creating Ollama client: {} with model {}", url, model));
                Box::new(OllamaClient::new(&url, &model))
            }
            LlmProvider::OpenAi => {
                if let Some(client) = OpenAiClient::from_env() {
                    XLog::i(TAG, "Creating OpenAI client from env");
                    Box::new(client)
                } else {
                    XLog::w(TAG, "OPENAI_API_KEY not set, falling back to Ollama");
                    Box::new(OllamaClient::new("http://localhost:11434", "llama3"))
                }
            }
            LlmProvider::Cascade => {
                XLog::i(TAG, "Creating CascadingLlmClient");
                let cascade = CascadingLlmClient::from_env();
                Box::new(cascade)
            }
            _ => {
                XLog::i(TAG, "Creating default Ollama client");
                Box::new(OllamaClient::new("http://localhost:11434", "llama3"))
            }
        }
    }
}
