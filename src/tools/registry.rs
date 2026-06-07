use std::collections::HashMap;
use crate::tools::types::{BaseTool, ToolResult};
use crate::tools::logic::screen::GetScreenInfoTool;
use crate::tools::logic::system::GetDeviceInfoTool;
use crate::tools::logic::input::InputTextTool;
use crate::tools::logic::notifications::GetNotificationsTool;
use crate::tools::logic::clipboard::ClipboardTool;
use crate::tools::logic::apps::GetInstalledAppsTool;
use crate::tools::logic::open_app::OpenAppTool;
use crate::tools::logic::wait::WaitTool;
use crate::tools::logic::finish::FinishTool;
use crate::tools::logic::calls::MakeCallTool;
use crate::tools::logic::messages::SendMessageTool;
use crate::tools::logic::google_calendar::GoogleCalendarTool;
use crate::tools::logic::google_drive::GoogleDriveTool;
use crate::tools::logic::machines::{
    ChronosTool, TechneTool, SomaTool, ElenchusTool, LexTool, HarmonyTool, PolisTool,
};
use crate::utils::logger::XLog;

const TAG: &str = "ToolRegistry";

pub enum DeviceType {
    Tv,
    Mobile,
}

pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn BaseTool>>,
    pub device_type: DeviceType,
}

impl ToolRegistry {
    pub fn new() -> Self {
        let mut registry = Self { tools: HashMap::new(), device_type: DeviceType::Mobile };
        registry.register_common_tools();
        registry
    }

    fn register_common_tools(&mut self) {
        self.register(Box::new(GetScreenInfoTool));
        self.register(Box::new(GetDeviceInfoTool));
        self.register(Box::new(InputTextTool));
        self.register(Box::new(GetNotificationsTool));
        self.register(Box::new(ClipboardTool));
        self.register(Box::new(GetInstalledAppsTool));
        self.register(Box::new(OpenAppTool));
        self.register(Box::new(WaitTool));
        self.register(Box::new(FinishTool));
        self.register(Box::new(MakeCallTool));
        self.register(Box::new(SendMessageTool));
        self.register(Box::new(GoogleCalendarTool));
        self.register(Box::new(GoogleDriveTool));
        // Philosophical machines (physis_pro reducers).
        self.register(Box::new(ChronosTool));
        self.register(Box::new(TechneTool));
        self.register(Box::new(SomaTool));
        self.register(Box::new(ElenchusTool));
        self.register(Box::new(LexTool));
        self.register(Box::new(HarmonyTool));
        self.register(Box::new(PolisTool));
    }

    pub fn register(&mut self, tool: Box<dyn BaseTool>) {
        XLog::i(TAG, &format!("Registering tool: {}", tool.name()));
        self.tools.insert(tool.name(), tool);
    }

    pub fn get_tool(&self, name: &str) -> Option<&dyn BaseTool> {
        self.tools.get(name).map(|t| t.as_ref())
    }

    pub fn get_display_name(&self, name: &str) -> String {
        self.tools.get(name).map(|t| t.display_name()).unwrap_or_else(|| name.to_string())
    }

    pub fn get_all_tools(&self) -> Vec<&dyn BaseTool> {
        self.tools.values().map(|t| t.as_ref()).collect()
    }

    pub fn execute_tool(&self, name: &str, params: HashMap<String, serde_json::Value>) -> ToolResult {
        match self.tools.get(name) {
            Some(tool) => {
                XLog::i(TAG, &format!("Executing: {}", name));
                tool.execute(params)
            }
            None => ToolResult::error(&format!("Unknown tool: {}", name)),
        }
    }

    pub fn execute_tool_with_wait(&self, name: &str, params: HashMap<String, serde_json::Value>) -> ToolResult {
        self.execute_tool(name, params)
    }
}
