use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use crate::agent::callback::AgentCallback;
use crate::agent::config::AgentConfig;
use crate::agent::guards::{InAppSearchGuard, EmailComposeGuard, DirectDeviceDataGuard};
use crate::agent::llm::client::{ChatMessage, LlmClient, LlmResponse, ToolSpecification};
use crate::agent::playbook::PlaybookManager;
use crate::agent::service::AgentService;
use crate::agent::skill::executor::SkillExecutor;
use crate::agent::skill::registry::SkillRegistry;
use crate::agent::stuck_detector::StuckDetector;
use crate::agent::task_budget::TaskBudget;
use crate::agent::task_prompt::TaskPromptEnvelope;
use crate::agent::token_monitor::TokenMonitor;
use crate::knowledge::kb_manager::KBManager;
use crate::memory::memory_manager::MemoryManager;
use crate::physis::client::PhysisClient;
use crate::tools::registry::ToolRegistry;
use crate::utils::logger::XLog;
use tokio::runtime::Runtime;

const TAG: &str = "DefaultAgentService";

pub struct DefaultAgentService {
    config: AgentConfig,
    llm_client: Option<Box<dyn LlmClient>>,
    tool_registry: Option<ToolRegistry>,
    skill_registry: Option<SkillRegistry>,
    memory_manager: Option<MemoryManager>,
    kb_manager: Option<KBManager>,
    physis_client: Option<PhysisClient>,
    running: Arc<AtomicBool>,
}

impl DefaultAgentService {
    pub fn new() -> Self {
        Self {
            config: AgentConfig::new(),
            llm_client: None,
            tool_registry: None,
            skill_registry: None,
            memory_manager: None,
            kb_manager: None,
            physis_client: None,
            running: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn with_llm_client(mut self, client: Box<dyn LlmClient>) -> Self {
        self.llm_client = Some(client);
        self
    }

    pub fn with_tool_registry(mut self, registry: ToolRegistry) -> Self {
        self.tool_registry = Some(registry);
        self
    }

    pub fn with_skill_registry(mut self, registry: SkillRegistry) -> Self {
        self.skill_registry = Some(registry);
        self
    }

    pub fn with_memory_manager(mut self, mm: MemoryManager) -> Self {
        self.memory_manager = Some(mm);
        self
    }

    pub fn with_kb_manager(mut self, kb: KBManager) -> Self {
        self.kb_manager = Some(kb);
        self
    }

    pub fn with_physis(mut self, physis: PhysisClient) -> Self {
        self.physis_client = Some(physis);
        self
    }

    fn get_tool_specs(tool_registry: &ToolRegistry) -> Vec<ToolSpecification> {
        tool_registry.get_all_tools().iter().map(|t| {
            let params = t.parameters().iter().map(|p| {
                (
                    p.name.clone(),
                    serde_json::json!({
                        "type": p.param_type,
                        "description": p.description,
                    })
                )
            }).collect::<serde_json::Map<_, _>>();

            ToolSpecification {
                name: t.name(),
                description: t.description(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": params,
                }),
            }
        }).collect()
    }

    fn build_system_prompt(&self, parsed: &TaskPromptEnvelope, tool_registry: &ToolRegistry) -> String {
        let mut parts: Vec<String> = Vec::new();

        if !self.config.system_prompt.is_empty() {
            parts.push(self.config.system_prompt.clone());
        }

        parts.push("You are an Android phone agent. You can use the following tools to interact with the device:".to_string());

        let tools_str: Vec<String> = tool_registry.get_all_tools().iter().map(|t| {
            let params: Vec<String> = t.parameters().iter().map(|p| {
                format!("  - {} ({}): {} {}", p.name, p.param_type, p.description, if p.required { "[REQUIRED]" } else { "[optional]" })
            }).collect();
            format!("{}. {}\n{}", t.display_name(), t.description(), params.join("\n"))
        }).collect();
        parts.push(tools_str.join("\n"));

        parts.push("\nAlways respond with a single tool call or a final answer. Never guess device data.".to_string());

        if parsed.has_background_state {
            if let Some(bg) = &parsed.background_state {
                parts.push(format!("\n[Background State]\n{}", bg));
            }
        }

        parts.join("\n\n")
    }

    fn inject_memory(&self, parsed: &TaskPromptEnvelope, _tool_registry: &ToolRegistry) -> String {
        let mut context = String::new();

        let search_query = parsed.current_request.clone();

        if let Some(kb) = &self.kb_manager {
            let vault_results = kb.search(&search_query);
            if !vault_results.is_empty() {
                context.push_str("\n[Relevant Memory from Vault]\n");
                for entry in vault_results.iter().take(3) {
                    context.push_str(&format!("- {}: {}\n", entry.title, entry.snippet));
                }
            }
        }

        if let Some(mm) = &self.memory_manager {
            let mem_results = mm.search(&search_query);
            if !mem_results.is_empty() {
                context.push_str("\n[Relevant Memory from Store]\n");
                for entry in mem_results.iter().take(3) {
                    context.push_str(&format!("- {}: {}\n", entry.topic, entry.content));
                }
            }
        }

        if let Some(physis) = &self.physis_client {
            if PhysisClient::is_running_http() {
                let rt = Runtime::new().unwrap();
                match rt.block_on(physis.query(&search_query)) {
                    Ok(result) => {
                        context.push_str("\n[Physis Semantic Context]\n");
                        context.push_str(&result);
                    }
                    Err(e) => {
                        XLog::w(TAG, &format!("Physis query failed: {}", e));
                    }
                }
            }
        }

        let playbook_mgr = PlaybookManager::new();
        if let Some(pb) = playbook_mgr.match_playbook(&search_query) {
            context.push_str("\n[Matched Playbook]\n");
            context.push_str(&format!("{}\n{}\n", pb.name, pb.body));
        }

        context
    }
}

impl AgentService for DefaultAgentService {
    fn initialize(&mut self, config: &AgentConfig) {
        XLog::i(TAG, "Initializing DefaultAgentService");
        self.config = config.clone();

        if self.llm_client.is_none() {
            let client = crate::agent::llm::factory::LlmClientFactory::create(config);
            self.llm_client = Some(client);
        }
        if self.tool_registry.is_none() {
            self.tool_registry = Some(ToolRegistry::new());
        }
        if self.kb_manager.is_none() {
            self.kb_manager = Some(KBManager::new());
        }
    }

    fn update_config(&mut self, config: &AgentConfig) {
        self.config = config.clone();
    }

    fn execute_task(&self, user_prompt: &str, callback: &dyn AgentCallback) {
        self.running.store(true, Ordering::Relaxed);

        let llm = match &self.llm_client {
            Some(c) => c,
            None => {
                callback.on_error(0, "LLM client not initialized", 0);
                return;
            }
        };
        let tool_registry = match &self.tool_registry {
            Some(r) => r,
            None => {
                callback.on_error(0, "Tool registry not initialized", 0);
                return;
            }
        };

        let parsed = TaskPromptEnvelope::parse(user_prompt);
        let mut token_monitor = TokenMonitor::new(llm.provider_name());
        let mut stuck_detector = StuckDetector::new();
        let task_budget = TaskBudget::from_settings();
        let mut guards = TaskGuards::from_task(&parsed.current_request);

        // Ingest goal into Physis
        if let Some(physis) = &self.physis_client {
            if PhysisClient::is_running_http() {
                let rt = Runtime::new().unwrap();
                match rt.block_on(physis.ingest(&parsed.current_request, user_prompt)) {
                    Ok(_) => XLog::i(TAG, "Goal ingested into Physis"),
                    Err(e) => XLog::w(TAG, &format!("Physis ingest failed: {}", e)),
                }
            }
        }

        let mut final_answer = String::new();

        // Check for deterministic skill match (shortcut before LLM loop)
        let mut skill_shortcut = false;
        if let Some(skill_registry) = &self.skill_registry {
            if let Some(matched) = skill_registry.find_by_trigger(&parsed.current_request) {
                XLog::i(TAG, &format!("Skill match: {} — executing deterministically", matched.name));
                let skill_params: HashMap<String, String> = HashMap::new();
                let result = SkillExecutor::execute(matched, &skill_params, tool_registry);
                if result.success {
                    XLog::i(TAG, &format!("Skill completed: {}", result.message));
                    callback.on_loop_start(0);
                    callback.on_content(0, &result.message);
                    callback.on_complete(0, &result.message, 0, None);
                    final_answer = result.message.clone();
                    skill_shortcut = true;
                } else {
                    XLog::w(TAG, &format!("Skill failed, falling back to LLM: {}", result.message));
                }
            }
        }

        let context = self.inject_memory(&parsed, tool_registry);
        let system_prompt = self.build_system_prompt(&parsed, tool_registry);
        let full_system = if context.is_empty() {
            system_prompt.clone()
        } else {
            format!("{}\n\n{}", system_prompt, context)
        };

        let mut messages: Vec<ChatMessage> = Vec::new();
        messages.push(ChatMessage::system(&full_system));

        if let Some(chat_history) = &parsed.chat_history {
            messages.push(ChatMessage::user(chat_history));
        }

        messages.push(ChatMessage::user(&parsed.current_request));

        let tool_specs = Self::get_tool_specs(tool_registry);
        let mut consecutive_errors = 0;
        const MAX_CONSECUTIVE_ERRORS: usize = 5;

        if skill_shortcut {
            self.running.store(false, Ordering::Relaxed);
            return;
        }

        'agent_loop: for round in 0..self.config.max_iterations {
            if !self.running.load(Ordering::Relaxed) {
                XLog::w(TAG, "Task cancelled");
                break;
            }

            callback.on_loop_start(round);

            if let Some(block) = guards.maybe_block_finish() {
                // Blocked by guard - inject correction
                messages.push(ChatMessage::user(&format!("CORRECTION: {}", block)));
            }

            let rt = Runtime::new().unwrap();
            let response = if self.config.streaming {
                match rt.block_on(llm.chat_streaming(&messages, &tool_specs)) {
                    Ok(chunks) => {
                        consecutive_errors = 0;
                        let text: String = chunks.iter().map(|c| c.token.as_str()).collect();
                        LlmResponse {
                            text: Some(text),
                            tool_execution_requests: Vec::new(),
                            model_name: None,
                            token_usage: None,
                            has_tool_execution_requests: false,
                        }
                    }
                    Err(e) => {
                        XLog::e(TAG, &format!("LLM streaming error on round {}: {}", round, e));
                        callback.on_error(round, &format!("Stream error: {}", e), token_monitor.get_status().total_tokens);
                        consecutive_errors += 1;
                        if consecutive_errors >= MAX_CONSECUTIVE_ERRORS { break; }
                        std::thread::sleep(Duration::from_secs(2));
                        continue;
                    }
                }
            } else {
                match rt.block_on(llm.chat(&messages, &tool_specs)) {
                    Ok(r) => {
                        consecutive_errors = 0;
                        r
                    }
                    Err(e) => {
                        XLog::e(TAG, &format!("LLM error on round {}: {}", round, e));
                        callback.on_error(round, &format!("LLM error: {}", e), token_monitor.get_status().total_tokens);
                        consecutive_errors += 1;
                        if consecutive_errors >= MAX_CONSECUTIVE_ERRORS {
                            XLog::e(TAG, "Too many consecutive errors, stopping");
                            break;
                        }
                        std::thread::sleep(Duration::from_secs(2));
                        continue;
                    }
                }
            };

            let model_name = response.model_name.unwrap_or_else(|| "unknown".to_string());

            token_monitor.record(
                round,
                response.token_usage.as_ref().and_then(|t| t.input_tokens),
                response.token_usage.as_ref().and_then(|t| t.output_tokens),
                response.token_usage.as_ref().map(|t| t.total_token_count()),
            );

            callback.on_token_update(&token_monitor.get_status());

            let budget_status = task_budget.check(
                token_monitor.get_status().total_tokens,
                token_monitor.get_status().estimated_cost_usd,
            );

            match budget_status {
                crate::agent::task_budget::BudgetStatus::HardLimit => {
                    XLog::w(TAG, "Hard budget limit reached, stopping");
                    callback.on_error(round, "Hard budget limit reached", token_monitor.get_status().total_tokens);
                    break;
                }
                crate::agent::task_budget::BudgetStatus::SoftLimit => {
                    XLog::i(TAG, "Soft budget limit reached, continuing with caution");
                }
                _ => {}
            }

            if let Some(text) = &response.text {
                let clean_text = text.trim();
                if !clean_text.is_empty() {
                    callback.on_content(round, clean_text);

                    if is_final_answer(clean_text) {
                        final_answer = clean_text.to_string();
                        callback.on_complete(round, &final_answer, token_monitor.get_status().total_tokens, Some(&model_name));
                        break 'agent_loop;
                    }
                }
            }

            if response.has_tool_execution_requests && !response.tool_execution_requests.is_empty() {
                // Block text-only completion for guarded tasks
                if guards.has_active_guards() {
                    if let Some(correction) = guards.build_correction() {
                        messages.push(ChatMessage::user(&format!("CORRECTION: {}", correction)));
                        continue;
                    }
                }

                for tool_req in &response.tool_execution_requests {
                    let params: HashMap<String, serde_json::Value> = match serde_json::from_str(&tool_req.arguments) {
                        Ok(p) => p,
                        Err(e) => {
                            callback.on_error(round, &format!("Invalid tool args: {}", e), token_monitor.get_status().total_tokens);
                            continue;
                        }
                    };

                    callback.on_tool_call(round, &tool_req.id, &tool_req.name, &tool_req.arguments);

                    guards.record_tool_attempt(&tool_req.name, &params);

                    let result = tool_registry.execute_tool_with_wait(&tool_req.name, params.clone());
                    callback.on_tool_result(round, &tool_req.id, &tool_req.name, &tool_req.arguments, &result);

                    let content = if result.is_success {
                        result.data.unwrap_or_else(|| "Success".to_string())
                    } else {
                        format!("ERROR: {}", result.error.unwrap_or_else(|| "Unknown error".to_string()))
                    };

                    // Persist tool result to memory manager
                    if let Some(mm) = &self.memory_manager {
                        let mem_key = format!("tool:{}:{}", &tool_req.name, round);
                        if let Err(e) = mm.save(&mem_key, &content) {
                            XLog::w(TAG, &format!("Memory save failed: {}", e));
                        }
                    }

                    messages.push(ChatMessage::user(&format!("Tool '{}' returned:\n{}", &tool_req.name, content)));
                }
            } else {
                // LLM produced text but no final answer marker - loop continues
                // Check if stuck
                let screen_text_hash = round as i32;
                if let Some(signal) = stuck_detector.record(
                    "text_response".to_string(),
                    screen_text_hash,
                    0,
                    None,
                ) {
                    XLog::w(TAG, &format!("Stuck detected: {:?}", signal));
                    messages.push(ChatMessage::user(&format!(
                        "You seem stuck: {}. Consider a different approach or call finish().",
                        signal.description
                    )));
                }
            }
        }

        self.running.store(false, Ordering::Relaxed);

        // Persist final answer to memory
        if !final_answer.is_empty() {
            if let Some(mm) = &self.memory_manager {
                let mem_key = format!("task:{}", chrono::Utc::now().timestamp());
                if let Err(e) = mm.save(&mem_key, &final_answer) {
                    XLog::w(TAG, &format!("Final memory save failed: {}", e));
                }
            }
        }

        // Ingest completion into Physis
        if let Some(physis) = &self.physis_client {
            if PhysisClient::is_running_http() {
                let rt = Runtime::new().unwrap();
                let summary = if final_answer.is_empty() {
                    format!("Task failed after {} iterations", self.config.max_iterations)
                } else {
                    format!("Completed: {}", final_answer)
                };
                match rt.block_on(physis.ingest(&parsed.current_request, &summary)) {
                    Ok(_) => XLog::i(TAG, "Completion ingested into Physis"),
                    Err(e) => XLog::w(TAG, &format!("Physis completion ingest failed: {}", e)),
                }
            }
        }

        if final_answer.is_empty() {
            callback.on_error(self.config.max_iterations, "Max iterations reached", token_monitor.get_status().total_tokens);
        }
    }

    fn cancel(&self) {
        XLog::i(TAG, "Cancelling task");
        self.running.store(false, Ordering::Relaxed);
    }

    fn shutdown(&mut self) {
        XLog::i(TAG, "Shutting down");
        self.running.store(false, Ordering::Relaxed);
        if let Some(ref mut llm) = self.llm_client {
            llm.close();
        }
    }

    fn is_running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }
}

fn is_final_answer(text: &str) -> bool {
    let t = text.trim().to_lowercase();
    t.starts_with("final answer:") || t.starts_with("final:") || t.starts_with("answer:")
}

struct TaskGuards {
    in_app_search: Option<InAppSearchGuard>,
    email_compose: Option<EmailComposeGuard>,
    device_data: Option<DirectDeviceDataGuard>,
}

impl TaskGuards {
    fn from_task(task: &str) -> Self {
        Self {
            in_app_search: Some(InAppSearchGuard::from_task(task)),
            email_compose: Some(EmailComposeGuard::from_task(task)),
            device_data: Some(DirectDeviceDataGuard::from_task(task)),
        }
    }

    fn has_active_guards(&self) -> bool {
        self.in_app_search.as_ref().map(|g| g.should_block_text_only_completion()).unwrap_or(false)
            || self.email_compose.as_ref().map(|g| g.should_block_text_only_completion_with_msg()).unwrap_or(false)
            || self.device_data.as_ref().map(|g| g.should_block_text_only_completion()).unwrap_or(false)
    }

    fn build_correction(&self) -> Option<String> {
        if let Some(g) = &self.in_app_search {
            if g.should_block_text_only_completion() {
                return Some(g.build_completion_correction());
            }
        }
        if let Some(g) = &self.email_compose {
            if g.should_block_text_only_completion_with_msg() {
                return Some("Use compose/send tools to write the email.".to_string());
            }
        }
        if let Some(g) = &self.device_data {
            if g.should_block_text_only_completion() {
                return Some(g.build_completion_correction());
            }
        }
        None
    }

    fn record_tool_attempt(&mut self, tool_name: &str, params: &HashMap<String, serde_json::Value>) {
        if let Some(ref mut g) = self.in_app_search {
            g.record_successful_tool(tool_name, params);
        }
        if let Some(ref mut g) = self.email_compose {
            g.record_successful_tool(tool_name);
        }
        if let Some(ref mut g) = self.device_data {
            g.record_tool_attempt(tool_name);
        }
    }

    fn maybe_block_finish(&self) -> Option<String> {
        let is_blocked = self.in_app_search.as_ref().and_then(|g| g.maybe_block_finish(None))
            .or_else(|| self.email_compose.as_ref().and_then(|g| g.maybe_block_finish(None)))
            .or_else(|| self.device_data.as_ref().and_then(|g| g.maybe_block_finish()));

        if is_blocked.is_some() {
            return is_blocked;
        }

        // Check if we're on a system dialog
        None
    }
}
