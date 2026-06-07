use std::collections::HashMap;
use crate::tools::types::{BaseTool, ToolResult, ToolParameter};

pub struct OpenAppTool;

impl BaseTool for OpenAppTool {
    fn name(&self) -> String { "open_app".to_string() }
    fn display_name(&self) -> String { "Open App".to_string() }
    fn description(&self) -> String { "Open an application by package name".to_string() }
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![ToolParameter::new("package_name", "string", "Package name of the app", true)]
    }

    fn execute(&self, params: HashMap<String, serde_json::Value>) -> ToolResult {
        let pkg = params.get("package_name").and_then(|v| v.as_str()).unwrap_or("unknown");
        ToolResult::success(&format!("Opened app: {}", pkg))
    }
}
