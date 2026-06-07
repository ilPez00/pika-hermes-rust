use std::collections::HashMap;
use crate::tools::types::{BaseTool, ToolResult, ToolParameter};

pub struct MakeCallTool;

impl BaseTool for MakeCallTool {
    fn name(&self) -> String { "make_call".to_string() }
    fn display_name(&self) -> String { "Make Call".to_string() }
    fn description(&self) -> String { "Make a phone call to a contact or phone number".to_string() }
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![
            ToolParameter::new("phone_number", "string", "Phone number to call", true),
        ]
    }

    fn execute(&self, params: HashMap<String, serde_json::Value>) -> ToolResult {
        let number = params.get("phone_number").and_then(|v| v.as_str()).unwrap_or("unknown");
        ToolResult::success(&format!("Calling {}...", number))
    }
}
