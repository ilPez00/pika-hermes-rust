use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskPromptEnvelope {
    pub current_request: String,
    pub has_chat_history: bool,
    pub has_background_state: bool,
    pub chat_history: Option<String>,
    pub background_state: Option<String>,
}

impl TaskPromptEnvelope {
    pub fn parse(raw: &str) -> Self {
        let parts: Vec<&str> = raw.splitn(3, "---").collect();
        if parts.len() >= 3 {
            Self {
                background_state: Some(parts[0].trim().to_string()),
                chat_history: Some(parts[1].trim().to_string()),
                current_request: parts[2].trim().to_string(),
                has_chat_history: true,
                has_background_state: true,
            }
        } else if parts.len() == 2 {
            Self {
                background_state: None,
                chat_history: Some(parts[0].trim().to_string()),
                current_request: parts[1].trim().to_string(),
                has_chat_history: true,
                has_background_state: false,
            }
        } else {
            Self {
                background_state: None,
                chat_history: None,
                current_request: raw.trim().to_string(),
                has_chat_history: false,
                has_background_state: false,
            }
        }
    }
}
