use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenStatus {
    pub total_tokens: i32,
    pub input_tokens: i32,
    pub output_tokens: i32,
    pub estimated_cost_usd: f64,
    pub formatted_tokens: String,
    pub formatted_cost: String,
}

pub struct TokenMonitor {
    model_name: String,
    total_input_tokens: i32,
    total_output_tokens: i32,
    step_records: Vec<StepRecord>,
}

struct StepRecord {
    step: usize,
    input_tokens: Option<i32>,
    output_tokens: Option<i32>,
    total_tokens: Option<i32>,
}

impl TokenMonitor {
    pub fn new(model_name: &str) -> Self {
        Self {
            model_name: model_name.to_string(),
            total_input_tokens: 0,
            total_output_tokens: 0,
            step_records: Vec::new(),
        }
    }

    pub fn record(&mut self, step: usize, input_tokens: Option<i32>, output_tokens: Option<i32>, total_tokens: Option<i32>) {
        if let Some(inp) = input_tokens { self.total_input_tokens += inp; }
        if let Some(out) = output_tokens { self.total_output_tokens += out; }
        self.step_records.push(StepRecord { step, input_tokens, output_tokens, total_tokens });
    }

    pub fn get_status(&self) -> TokenStatus {
        let total = self.total_input_tokens + self.total_output_tokens;
        let cost = total as f64 * 0.000002;
        TokenStatus {
            total_tokens: total,
            input_tokens: self.total_input_tokens,
            output_tokens: self.total_output_tokens,
            estimated_cost_usd: cost,
            formatted_tokens: Self::format_tokens(total),
            formatted_cost: format!("${:.5}", cost),
        }
    }

    fn format_tokens(tokens: i32) -> String {
        if tokens >= 1_000_000 {
            format!("{:.1}M", tokens as f64 / 1_000_000.0)
        } else if tokens >= 1_000 {
            format!("{:.1}K", tokens as f64 / 1_000.0)
        } else {
            format!("{}", tokens)
        }
    }
}
