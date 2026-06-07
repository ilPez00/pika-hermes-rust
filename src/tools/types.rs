use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub is_success: bool,
    pub data: Option<String>,
    pub error: Option<String>,
}

impl ToolResult {
    pub fn success(data: &str) -> Self {
        Self { is_success: true, data: Some(data.to_string()), error: None }
    }

    pub fn error(msg: &str) -> Self {
        Self { is_success: false, data: None, error: Some(msg.to_string()) }
    }
}

#[derive(Debug, Clone)]
pub struct ToolParameter {
    pub name: String,
    pub param_type: String,
    pub description: String,
    pub required: bool,
}

impl ToolParameter {
    pub fn new(name: &str, param_type: &str, description: &str, required: bool) -> Self {
        Self {
            name: name.to_string(),
            param_type: param_type.to_string(),
            description: description.to_string(),
            required,
        }
    }
}

pub trait BaseTool: Send + Sync {
    fn name(&self) -> String;
    fn display_name(&self) -> String;
    fn description(&self) -> String;
    fn parameters(&self) -> Vec<ToolParameter>;
    fn execute(&self, params: HashMap<String, serde_json::Value>) -> ToolResult;
}
