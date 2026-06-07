use std::collections::HashMap;
use crate::tools::types::{BaseTool, ToolResult, ToolParameter};

pub struct GetNotificationsTool;

impl BaseTool for GetNotificationsTool {
    fn name(&self) -> String { "get_notifications".to_string() }
    fn display_name(&self) -> String { "Notifications".to_string() }
    fn description(&self) -> String { "Read current notifications from the notification shade".to_string() }
    fn parameters(&self) -> Vec<ToolParameter> { vec![] }

    fn execute(&self, _params: HashMap<String, serde_json::Value>) -> ToolResult {
        ToolResult::success("Notifications:\n- [WhatsApp] Mom: Can you pick up dinner?\n- [Gmail] Subscription renewal notice")
    }
}
