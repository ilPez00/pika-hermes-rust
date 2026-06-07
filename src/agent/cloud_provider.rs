#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CloudProvider {
    OpenAi,
    Anthropic,
    Google,
    Groq,
    DeepSeek,
    Together,
    Ollama,
    OpenRouter,
    Local,
}

impl CloudProvider {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "openai" | "gpt" => CloudProvider::OpenAi,
            "anthropic" | "claude" => CloudProvider::Anthropic,
            "google" | "gemini" => CloudProvider::Google,
            "groq" => CloudProvider::Groq,
            "deepseek" => CloudProvider::DeepSeek,
            "together" => CloudProvider::Together,
            "ollama" => CloudProvider::Ollama,
            "openrouter" => CloudProvider::OpenRouter,
            _ => CloudProvider::Local,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            CloudProvider::OpenAi => "OpenAI",
            CloudProvider::Anthropic => "Anthropic",
            CloudProvider::Google => "Google",
            CloudProvider::Groq => "Groq",
            CloudProvider::DeepSeek => "DeepSeek",
            CloudProvider::Together => "Together AI",
            CloudProvider::Ollama => "Ollama",
            CloudProvider::OpenRouter => "OpenRouter",
            CloudProvider::Local => "Local",
        }
    }
}
