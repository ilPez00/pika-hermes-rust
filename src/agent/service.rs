pub trait AgentService: Send {
    fn initialize(&mut self, config: &crate::agent::config::AgentConfig);
    fn update_config(&mut self, config: &crate::agent::config::AgentConfig);
    fn execute_task(&self, user_prompt: &str, callback: &dyn crate::agent::callback::AgentCallback);
    fn cancel(&self);
    fn shutdown(&mut self);
    fn is_running(&self) -> bool;
}
