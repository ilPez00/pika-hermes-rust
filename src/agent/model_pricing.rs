use crate::agent::cloud_provider::CloudProvider;

pub struct ModelPricing;

impl ModelPricing {
    pub fn input_price_per_1k(provider: CloudProvider, model: &str) -> f64 {
        match provider {
            CloudProvider::OpenAi => match model {
                m if m.contains("gpt-4o") => 0.0025,
                m if m.contains("gpt-4-turbo") => 0.01,
                m if m.contains("gpt-4") => 0.03,
                m if m.contains("gpt-3.5") => 0.0005,
                _ => 0.001,
            },
            CloudProvider::Anthropic => match model {
                m if m.contains("opus") => 0.015,
                m if m.contains("sonnet") => 0.003,
                m if m.contains("haiku") => 0.00025,
                _ => 0.003,
            },
            CloudProvider::Groq => match model {
                m if m.contains("llama3-70b") => 0.00059,
                m if m.contains("llama3-8b") => 0.00005,
                m if m.contains("mixtral") => 0.00027,
                _ => 0.0001,
            },
            CloudProvider::DeepSeek => match model {
                m if m.contains("deepseek-chat") => 0.00014,
                m if m.contains("deepseek-coder") => 0.00014,
                _ => 0.0001,
            },
            _ => 0.0,
        }
    }

    pub fn output_price_per_1k(provider: CloudProvider, model: &str) -> f64 {
        match provider {
            CloudProvider::OpenAi => match model {
                m if m.contains("gpt-4o") => 0.01,
                m if m.contains("gpt-4-turbo") => 0.03,
                m if m.contains("gpt-4") => 0.06,
                m if m.contains("gpt-3.5") => 0.0015,
                _ => 0.002,
            },
            CloudProvider::Anthropic => match model {
                m if m.contains("opus") => 0.075,
                m if m.contains("sonnet") => 0.015,
                m if m.contains("haiku") => 0.00125,
                _ => 0.015,
            },
            CloudProvider::Groq => match model {
                m if m.contains("llama3-70b") => 0.00079,
                m if m.contains("llama3-8b") => 0.00008,
                m if m.contains("mixtral") => 0.00027,
                _ => 0.0001,
            },
            CloudProvider::DeepSeek => match model {
                m if m.contains("deepseek-chat") => 0.00028,
                m if m.contains("deepseek-coder") => 0.00028,
                _ => 0.0001,
            },
            _ => 0.0,
        }
    }

    pub fn estimate_cost(provider: CloudProvider, model: &str, input_tokens: i32, output_tokens: i32) -> f64 {
        let input_cost = input_tokens as f64 * Self::input_price_per_1k(provider, model) / 1000.0;
        let output_cost = output_tokens as f64 * Self::output_price_per_1k(provider, model) / 1000.0;
        input_cost + output_cost
    }
}
