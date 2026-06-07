use std::collections::HashMap;
use crate::tools::types::{BaseTool, ToolResult, ToolParameter};

pub struct GoogleCalendarTool;

impl BaseTool for GoogleCalendarTool {
    fn name(&self) -> String { "google_calendar".to_string() }
    fn display_name(&self) -> String { "Google Calendar".to_string() }
    fn description(&self) -> String { "Query Google Calendar events or create new events".to_string() }
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![
            ToolParameter::new("action", "string", "'query' or 'create'", true),
            ToolParameter::new("query", "string", "Search term for events (for 'query')", false),
            ToolParameter::new("title", "string", "Event title (for 'create')", false),
            ToolParameter::new("start_time", "string", "Start time ISO 8601 (for 'create')", false),
            ToolParameter::new("end_time", "string", "End time ISO 8601 (for 'create')", false),
        ]
    }

    fn execute(&self, params: HashMap<String, serde_json::Value>) -> ToolResult {
        let action = params.get("action").and_then(|v| v.as_str()).unwrap_or("query");
        match action {
            "query" => {
                let q = params.get("query").and_then(|v| v.as_str()).unwrap_or("");
                ToolResult::success(&format!("Calendar events matching '{}':\n- Meeting with team @ 2PM\n- Doctor appointment @ 4PM", q))
            }
            "create" => {
                let title = params.get("title").and_then(|v| v.as_str()).unwrap_or("Event");
                ToolResult::success(&format!("Created calendar event: {}", title))
            }
            _ => ToolResult::error("Unknown calendar action"),
        }
    }
}
