use std::collections::HashMap;
use crate::agent::skill::skill::*;
use crate::tools::registry::ToolRegistry;
use crate::utils::logger::XLog;

const TAG: &str = "SkillExecutor";

pub struct SkillExecutor;

impl SkillExecutor {
    pub fn execute(
        skill: &Skill,
        params: &HashMap<String, String>,
        tool_registry: &ToolRegistry,
    ) -> SkillResult {
        XLog::i(TAG, &format!("Executing skill: {} with params: {:?}", skill.id, params));
        let total_steps = skill.steps.len();
        let mut steps_used = 0;

        for (index, step) in skill.steps.iter().enumerate() {
            steps_used = index + 1;
            XLog::d(TAG, &format!("Step {}/{}: {} — {}", steps_used, total_steps, step.tool_name, step.description));

            let resolved = resolve_params(&step.params, params);
            let mut succeeded = false;

            for attempt in 1..=step.retries {
                let tool_params: HashMap<String, serde_json::Value> = resolved.iter()
                    .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
                    .collect();

                let result = tool_registry.execute_tool_with_wait(&step.tool_name, tool_params);

                if result.is_success {
                    XLog::d(TAG, &format!("Step {} OK: {:?}", steps_used, result.data.as_ref().map(|s| &s[..s.len().min(100)])));
                    succeeded = true;
                    break;
                } else {
                    XLog::w(TAG, &format!("Step {} failed (attempt {}/{}): {:?}",
                        steps_used, attempt, step.retries, result.error));
                    if attempt < step.retries {
                        std::thread::sleep(std::time::Duration::from_millis(500));
                    }
                }
            }

            if !succeeded {
                if step.optional {
                    XLog::i(TAG, &format!("Step {} failed but optional, continuing", steps_used));
                    continue;
                }
                XLog::w(TAG, &format!("Skill {} failed at step {}: {}", skill.id, steps_used, step.description));
                return SkillResult::fail(steps_used,
                    &format!("Failed at step {}: {}. Fallback goal: {}",
                        steps_used, step.description, resolve_template(&skill.fallback_goal, params)));
            }
        }

        XLog::i(TAG, &format!("Skill {} completed in {} steps", skill.id, steps_used));
        SkillResult::ok(steps_used, &format!("Skill '{}' completed successfully in {} steps.", skill.name, steps_used))
    }
}

fn resolve_params(step_params: &HashMap<String, String>, user_params: &HashMap<String, String>) -> HashMap<String, String> {
    step_params.iter().map(|(k, v)| {
        (k.clone(), resolve_template(v, user_params))
    }).collect()
}

fn resolve_template(template: &str, params: &HashMap<String, String>) -> String {
    let mut result = template.to_string();
    for (key, value) in params {
        result = result.replace(&format!("{{{}}}", key), value);
    }
    result
}
