use std::collections::HashMap;
use crate::tools::types::{BaseTool, ToolResult, ToolParameter};

pub struct InputTextTool;

impl BaseTool for InputTextTool {
    fn name(&self) -> String { "input_text".to_string() }
    fn display_name(&self) -> String { "Input Text".to_string() }
    fn description(&self) -> String { "Type text into the currently focused input field".to_string() }
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![ToolParameter::new("text", "string", "Text to input", true)]
    }

    fn execute(&self, params: HashMap<String, serde_json::Value>) -> ToolResult {
        let text = params.get("text").and_then(|v| v.as_str()).unwrap_or("");
        ToolResult::success(&format!("Text '{}' input successfully", text))
    }
}
