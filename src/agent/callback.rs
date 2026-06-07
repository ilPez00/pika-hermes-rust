use crate::tools::types::ToolResult;
use crate::agent::token_monitor::TokenStatus;

pub trait AgentCallback: Send {
    fn on_loop_start(&self, round: usize);
    fn on_content(&self, round: usize, content: &str);
    fn on_tool_call(&self, round: usize, tool_id: &str, tool_name: &str, parameters: &str);
    fn on_tool_result(&self, round: usize, tool_id: &str, tool_name: &str, parameters: &str, result: &ToolResult);
    fn on_token_update(&self, status: &TokenStatus);
    fn on_complete(&self, round: usize, final_answer: &str, total_tokens: i32, model_name: Option<&str>);
    fn on_error(&self, round: usize, error: &str, total_tokens: i32);
    fn on_system_dialog_blocked(&self, round: usize, total_tokens: i32);
}

pub struct DefaultAgentCallback;

impl AgentCallback for DefaultAgentCallback {
    fn on_loop_start(&self, round: usize) {
        println!("[Agent] Round {} starting", round);
    }
    fn on_content(&self, round: usize, content: &str) {
        println!("[Agent] Round {}: {}", round, content);
    }
    fn on_tool_call(&self, round: usize, tool_id: &str, tool_name: &str, parameters: &str) {
        println!("[Agent] Round {}: {} ({}) <- {}", round, tool_name, tool_id, parameters);
    }
    fn on_tool_result(&self, round: usize, _tool_id: &str, tool_name: &str, _parameters: &str, result: &ToolResult) {
        println!("[Agent] Round {}: {} result: {}", round, tool_name, if result.is_success { "OK" } else { "FAIL" });
    }
    fn on_token_update(&self, status: &TokenStatus) {
        println!("[Agent] Tokens: {}", status.formatted_tokens);
    }
    fn on_complete(&self, _round: usize, final_answer: &str, total_tokens: i32, _model_name: Option<&str>) {
        println!("[Agent] Complete ({} tokens): {}", total_tokens, final_answer);
    }
    fn on_error(&self, _round: usize, error: &str, total_tokens: i32) {
        eprintln!("[Agent] Error ({} tokens): {}", total_tokens, error);
    }
    fn on_system_dialog_blocked(&self, _round: usize, _total_tokens: i32) {
        eprintln!("[Agent] System dialog blocked");
    }
}
