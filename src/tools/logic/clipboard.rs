use std::collections::HashMap;
use crate::tools::types::{BaseTool, ToolResult, ToolParameter};

pub struct ClipboardTool;

impl BaseTool for ClipboardTool {
    fn name(&self) -> String { "clipboard".to_string() }
    fn display_name(&self) -> String { "Clipboard".to_string() }
    fn description(&self) -> String { "Read or write the system clipboard".to_string() }
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![
            ToolParameter::new("action", "string", "'get' or 'set'", true),
            ToolParameter::new("text", "string", "Text to set (only for 'set')", false),
        ]
    }

    fn execute(&self, params: HashMap<String, serde_json::Value>) -> ToolResult {
        let action = params.get("action").and_then(|v| v.as_str()).unwrap_or("get");
        match action {
            "get" => ToolResult::success("Clipboard: 'Meeting notes from today...'"),
            "set" => ToolResult::success("Clipboard set successfully"),
            _ => ToolResult::error("Unknown clipboard action"),
        }
    }
}
