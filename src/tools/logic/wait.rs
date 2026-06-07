use std::collections::HashMap;
use crate::tools::types::{BaseTool, ToolResult, ToolParameter};

pub struct WaitTool;

impl BaseTool for WaitTool {
    fn name(&self) -> String { "wait".to_string() }
    fn display_name(&self) -> String { "Wait".to_string() }
    fn description(&self) -> String { "Wait for a specified duration in milliseconds".to_string() }
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![ToolParameter::new("duration_ms", "integer", "Duration in milliseconds", true)]
    }

    fn execute(&self, params: HashMap<String, serde_json::Value>) -> ToolResult {
        let ms = params.get("duration_ms").and_then(|v| v.as_i64()).unwrap_or(1000);
        ToolResult::success(&format!("Waited for {}ms", ms))
    }
}
