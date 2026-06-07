use std::collections::HashMap;
use crate::tools::types::{BaseTool, ToolResult, ToolParameter};

pub struct GetDeviceInfoTool;

impl BaseTool for GetDeviceInfoTool {
    fn name(&self) -> String { "get_device_info".to_string() }
    fn display_name(&self) -> String { "Device Info".to_string() }
    fn description(&self) -> String { "Get device system info for any category: battery, wifi, storage, bluetooth, screen, device, time".to_string() }
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![ToolParameter::new("category", "string", "Info category", true)]
    }

    fn execute(&self, params: HashMap<String, serde_json::Value>) -> ToolResult {
        let category = params.get("category")
            .and_then(|v| v.as_str())
            .unwrap_or("device")
            .to_lowercase();

        match category.as_str() {
            "battery" => ToolResult::success("Battery: 85%, charging, 37.2°C"),
            "wifi" => ToolResult::success("WiFi: connected to 'HomeNetwork', 5GHz, -45dBm"),
            "storage" => ToolResult::success("Storage: 32.5GB used of 128GB (25%), 95.5GB free"),
            "bluetooth" => ToolResult::success("Bluetooth: enabled, paired devices: Headphones, Speaker"),
            "screen" => ToolResult::success("Brightness: 65%, Dark mode: OFF, Auto-brightness: ON"),
            "device" => ToolResult::success("Android 14 (API 34), Model: Google Pixel 8"),
            "time" => ToolResult::success("Current time: 2026-05-31 16:30:00 UTC+2"),
            _ => ToolResult::error(&format!("Unknown category: {}", category)),
        }
    }
}
