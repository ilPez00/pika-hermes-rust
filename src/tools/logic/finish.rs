use std::collections::HashMap;
use crate::tools::types::{BaseTool, ToolResult, ToolParameter};

pub struct FinishTool;

impl BaseTool for FinishTool {
    fn name(&self) -> String { "finish".to_string() }
    fn display_name(&self) -> String { "Finish Task".to_string() }
    fn description(&self) -> String { "Signal that the task is complete with a summary of what was done".to_string() }
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![ToolParameter::new("summary", "string", "Summary of what was accomplished", true)]
    }

    fn execute(&self, params: HashMap<String, serde_json::Value>) -> ToolResult {
        let summary = params.get("summary").and_then(|v| v.as_str()).unwrap_or("Task completed");
        ToolResult::success(summary)
    }
}
