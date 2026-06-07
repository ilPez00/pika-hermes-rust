use crate::agent::llm::client::LlmProvider;

#[derive(Clone)]
pub struct AgentConfig {
    pub provider: LlmProvider,
    pub model_name: Option<String>,
    pub system_prompt: String,
    pub streaming: bool,
    pub max_iterations: usize,
    pub temperature: f32,
}

impl AgentConfig {
    pub fn new() -> Self {
        Self {
            provider: LlmProvider::Ollama,
            model_name: Some("llama3".into()),
            system_prompt: String::new(),
            streaming: false,
            max_iterations: 20,
            temperature: 0.7,
        }
    }

    pub fn with_provider(mut self, provider: LlmProvider) -> Self {
        self.provider = provider;
        self
    }

    pub fn with_model(mut self, model: &str) -> Self {
        self.model_name = Some(model.to_string());
        self
    }

    pub fn with_system_prompt(mut self, prompt: &str) -> Self {
        self.system_prompt = prompt.to_string();
        self
    }
}
