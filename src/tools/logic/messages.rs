use std::collections::HashMap;
use crate::tools::types::{BaseTool, ToolResult, ToolParameter};

pub struct SendMessageTool;

impl BaseTool for SendMessageTool {
    fn name(&self) -> String { "send_message".to_string() }
    fn display_name(&self) -> String { "Send Message".to_string() }
    fn description(&self) -> String { "Send a text message via SMS or messaging app".to_string() }
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![
            ToolParameter::new("phone_number", "string", "Phone number", true),
            ToolParameter::new("message", "string", "Message text", true),
        ]
    }

    fn execute(&self, params: HashMap<String, serde_json::Value>) -> ToolResult {
        let number = params.get("phone_number").and_then(|v| v.as_str()).unwrap_or("unknown");
        let message = params.get("message").and_then(|v| v.as_str()).unwrap_or("");
        ToolResult::success(&format!("Message sent to {}: {}", number, message))
    }
}
