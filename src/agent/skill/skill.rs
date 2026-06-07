use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum SkillCategory {
    Input,
    Navigation,
    Messaging,
    Dismiss,
    Media,
    General,
}

impl SkillCategory {
    pub fn label(&self) -> &str {
        match self {
            SkillCategory::Input => "Input",
            SkillCategory::Navigation => "Navigation",
            SkillCategory::Messaging => "Messaging",
            SkillCategory::Dismiss => "Dismiss",
            SkillCategory::Media => "Media",
            SkillCategory::General => "General",
        }
    }
}

#[derive(Debug, Clone)]
pub struct SkillStep {
    pub tool_name: String,
    pub params: HashMap<String, String>,
    pub description: String,
    pub optional: bool,
    pub retries: usize,
}

impl SkillStep {
    pub fn new(tool_name: &str) -> Self {
        Self {
            tool_name: tool_name.to_string(),
            params: HashMap::new(),
            description: String::new(),
            optional: false,
            retries: 1,
        }
    }

    pub fn with_params(mut self, params: Vec<(&str, &str)>) -> Self {
        self.params = params.into_iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
        self
    }

    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }
}

#[derive(Debug, Clone)]
pub struct SkillParameter {
    pub name: String,
    pub param_type: String,
    pub required: bool,
    pub description: String,
    pub default_value: String,
}

impl SkillParameter {
    pub fn new(name: &str, required: bool) -> Self {
        Self {
            name: name.to_string(),
            param_type: "string".to_string(),
            required,
            description: String::new(),
            default_value: String::new(),
        }
    }

    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    pub fn with_default(mut self, val: &str) -> Self {
        self.default_value = val.to_string();
        self
    }
}

#[derive(Debug, Clone)]
pub struct SkillResult {
    pub success: bool,
    pub steps_used: usize,
    pub tokens_used: usize,
    pub fallback_used: bool,
    pub message: String,
}

impl SkillResult {
    pub fn ok(steps: usize, msg: &str) -> Self {
        Self { success: true, steps_used: steps, tokens_used: 0, fallback_used: false, message: msg.to_string() }
    }

    pub fn fail(steps: usize, msg: &str) -> Self {
        Self { success: false, steps_used: steps, tokens_used: 0, fallback_used: false, message: msg.to_string() }
    }
}

#[derive(Debug, Clone)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: SkillCategory,
    pub estimated_steps_saved: usize,
    pub steps: Vec<SkillStep>,
    pub parameters: Vec<SkillParameter>,
    pub trigger_patterns: Vec<String>,
    pub fallback_goal: String,
    pub user_facing: bool,
}

impl Skill {
    pub fn new(id: &str, name: &str, description: &str, category: SkillCategory) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            category,
            estimated_steps_saved: 5,
            steps: Vec::new(),
            parameters: Vec::new(),
            trigger_patterns: Vec::new(),
            fallback_goal: String::new(),
            user_facing: false,
        }
    }

    pub fn with_steps(mut self, steps: Vec<SkillStep>) -> Self {
        self.steps = steps;
        self
    }

    pub fn with_parameters(mut self, params: Vec<SkillParameter>) -> Self {
        self.parameters = params;
        self
    }

    pub fn with_triggers(mut self, triggers: Vec<&str>) -> Self {
        self.trigger_patterns = triggers.into_iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn with_fallback(mut self, goal: &str) -> Self {
        self.fallback_goal = goal.to_string();
        self
    }

    pub fn user_facing(mut self) -> Self {
        self.user_facing = true;
        self
    }

    pub fn save_steps(mut self, n: usize) -> Self {
        self.estimated_steps_saved = n;
        self
    }
}
