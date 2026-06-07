use std::collections::HashMap;
use crate::tools::types::{BaseTool, ToolResult, ToolParameter};

pub struct GetInstalledAppsTool;

impl BaseTool for GetInstalledAppsTool {
    fn name(&self) -> String { "get_installed_apps".to_string() }
    fn display_name(&self) -> String { "Installed Apps".to_string() }
    fn description(&self) -> String { "List all installed applications on the device".to_string() }
    fn parameters(&self) -> Vec<ToolParameter> { vec![] }

    fn execute(&self, _params: HashMap<String, serde_json::Value>) -> ToolResult {
        ToolResult::success("Installed apps: WhatsApp, Gmail, Chrome, YouTube, Spotify, Maps, Settings, Camera, Phone, Messages")
    }
}
