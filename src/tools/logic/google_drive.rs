use std::collections::HashMap;
use crate::tools::types::{BaseTool, ToolResult, ToolParameter};

pub struct GoogleDriveTool;

impl BaseTool for GoogleDriveTool {
    fn name(&self) -> String { "google_drive".to_string() }
    fn display_name(&self) -> String { "Google Drive".to_string() }
    fn description(&self) -> String { "Search Google Drive files or read file contents".to_string() }
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![
            ToolParameter::new("action", "string", "'search' or 'read' or 'list'", true),
            ToolParameter::new("query", "string", "Search query (for 'search')", false),
            ToolParameter::new("file_id", "string", "File ID (for 'read')", false),
        ]
    }

    fn execute(&self, params: HashMap<String, serde_json::Value>) -> ToolResult {
        let action = params.get("action").and_then(|v| v.as_str()).unwrap_or("list");
        match action {
            "search" => {
                let q = params.get("query").and_then(|v| v.as_str()).unwrap_or("");
                ToolResult::success(&format!("Drive files matching '{}':\n- Budget.xlsx\n- Notes.md", q))
            }
            "read" => {
                let fid = params.get("file_id").and_then(|v| v.as_str()).unwrap_or("unknown");
                ToolResult::success(&format!("File {} contents: Sample file content...", fid))
            }
            "list" => ToolResult::success("Recent Drive files:\n- Budget.xlsx\n- Meeting Notes.md\n- Report.pdf"),
            _ => ToolResult::error("Unknown drive action"),
        }
    }
}
