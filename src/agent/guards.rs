const TAG: &str = "TaskGuards";

pub struct InAppSearchGuard {
    blocked: bool,
}

impl InAppSearchGuard {
    pub fn from_task(task: &str) -> Self {
        let blocked = task.to_lowercase().contains("search") && !task.to_lowercase().contains("web search");
        Self { blocked }
    }

    pub fn should_block_text_only_completion(&self) -> bool {
        self.blocked
    }

    pub fn build_completion_correction(&self) -> String {
        "You must use the in-app search tool to find this information. Do not guess.".to_string()
    }

    pub fn record_successful_tool(&mut self, tool_name: &str, _params: &std::collections::HashMap<String, serde_json::Value>) {
        if tool_name.contains("search") {
            self.blocked = false;
        }
    }

    pub fn maybe_block_finish(&self, _screen_info: Option<&str>) -> Option<String> {
        if self.blocked {
            Some("You haven't performed the search yet. Use the search tool first.".to_string())
        } else {
            None
        }
    }
}

pub struct EmailComposeGuard {
    blocked: bool,
}

impl EmailComposeGuard {
    pub fn from_task(task: &str) -> Self {
        let blocked = task.to_lowercase().contains("email") || task.to_lowercase().contains("compose");
        Self { blocked }
    }

    pub fn should_block_text_only_completion(&self) -> bool {
        self.blocked
    }

    pub fn should_block_text_only_completion_with_msg(&self) -> bool {
        self.blocked
    }

    pub fn record_successful_tool(&mut self, tool_name: &str) {
        if tool_name.contains("email") || tool_name.contains("send") {
            self.blocked = false;
        }
    }

    pub fn maybe_block_finish(&self, _screen_info: Option<&str>) -> Option<String> {
        if self.blocked {
            Some("You haven't composed and sent the email yet. Use the compose tool.".to_string())
        } else {
            None
        }
    }
}

pub struct DirectDeviceDataGuard {
    blocked: bool,
}

impl DirectDeviceDataGuard {
    pub fn from_task(task: &str) -> Self {
        let keywords = ["battery", "wifi", "storage", "time", "date", "notification"];
        let blocked = keywords.iter().any(|k| task.to_lowercase().contains(k));
        Self { blocked }
    }

    pub fn should_block_text_only_completion(&self) -> bool {
        self.blocked
    }

    pub fn build_completion_correction(&self) -> String {
        "Use get_device_info() or get_notifications() to fetch this data directly. Do not guess.".to_string()
    }

    pub fn record_tool_attempt(&mut self, tool_name: &str) {
        if tool_name == "get_device_info" || tool_name == "get_notifications" {
            self.blocked = false;
        }
    }

    pub fn maybe_block_finish(&self) -> Option<String> {
        if self.blocked {
            Some("Use the device info tool to get this data.".to_string())
        } else {
            None
        }
    }
}
