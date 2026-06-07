use std::collections::HashMap;
use crate::tools::types::{BaseTool, ToolResult, ToolParameter};

pub struct GetScreenInfoTool;

impl BaseTool for GetScreenInfoTool {
    fn name(&self) -> String { "get_screen_info".to_string() }
    fn display_name(&self) -> String { "Screen Info".to_string() }
    fn description(&self) -> String { "Get the current screen content and UI hierarchy".to_string() }
    fn parameters(&self) -> Vec<ToolParameter> { vec![] }

    fn execute(&self, _params: HashMap<String, serde_json::Value>) -> ToolResult {
        ToolResult::success("Screen: Home Screen\n- Status Bar: WiFi, 4:30PM\n- Notifications: 2 unread")
    }
}
