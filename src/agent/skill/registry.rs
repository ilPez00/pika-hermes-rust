use crate::agent::skill::skill::*;
use crate::agent::skill::built_in::BuiltInSkills;
use crate::utils::logger::XLog;
use std::collections::HashMap;

const TAG: &str = "SkillRegistry";

pub struct SkillRegistry {
    skills: HashMap<String, Skill>,
}

impl SkillRegistry {
    pub fn new() -> Self {
        let mut registry = Self { skills: HashMap::new() };
        registry.load_built_in();
        registry
    }

    pub fn register(&mut self, skill: Skill) {
        XLog::d(TAG, &format!("Registered skill: {} ({})", skill.id, skill.name));
        self.skills.insert(skill.id.clone(), skill);
    }

    pub fn find_by_id(&self, id: &str) -> Option<&Skill> {
        self.skills.get(id)
    }

    pub fn get_all(&self) -> Vec<&Skill> {
        self.skills.values().collect()
    }

    pub fn get_by_category(&self, category: &SkillCategory) -> Vec<&Skill> {
        self.skills.values().filter(|s| matches_category(&s.category, category)).collect()
    }

    pub fn get_user_facing(&self) -> Vec<&Skill> {
        self.skills.values().filter(|s| s.user_facing).collect()
    }

    pub fn find_by_trigger(&self, task: &str) -> Option<&Skill> {
        let lower = task.to_lowercase();

        // Compound tasks should go to agent loop, not skills
        if lower.contains(" and ") || lower.contains(" then ") || lower.contains(" after ") {
            XLog::d(TAG, &format!("Compound task detected, skipping skill matching: {}", task));
            return None;
        }

        for skill in self.skills.values() {
            for pattern in &skill.trigger_patterns {
                if lower.contains(&pattern.to_lowercase()) {
                    return Some(skill);
                }
            }
        }
        None
    }

    fn load_built_in(&mut self) {
        for skill in BuiltInSkills::all() {
            self.register(skill);
        }
        XLog::i(TAG, &format!("Loaded {} built-in skills", self.skills.len()));
    }
}

fn matches_category(a: &SkillCategory, b: &SkillCategory) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}
