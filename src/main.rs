mod agent;
mod knowledge;
mod memory;
mod physis;
mod tools;
mod utils;

use agent::callback::DefaultAgentCallback;
use agent::config::AgentConfig;
use agent::default_service::DefaultAgentService;
use agent::llm::client::LlmProvider;
use memory::memory_manager::MemoryManager;
use agent::service::AgentService;
use utils::logger::XLog;

const TAG: &str = "PikaHermes";

fn main() {
    env_logger::init();
    XLog::i(TAG, "Pika-Hermes Rust starting");

    let args: Vec<String> = std::env::args().collect();
    let prompt = if args.len() > 1 {
        args[1..].join(" ")
    } else {
        "Hello, what can you do?".to_string()
    };

    let provider = std::env::var("LLM_PROVIDER")
        .map(|s| LlmProvider::from_str(&s))
        .unwrap_or(LlmProvider::Ollama);

    let config = AgentConfig::new()
        .with_provider(provider)
        .with_model(&std::env::var("LLM_MODEL").unwrap_or_else(|_| "llama3".into()))
        .with_system_prompt("You are Pika-Hermes, an Android phone agent.");

    let mut agent = DefaultAgentService::new();

    let memory_path = std::env::var("MEMORY_PATH").unwrap_or_else(|_| "pika_memory.sled".into());
    if let Ok(mm) = MemoryManager::new(&memory_path) {
        agent = DefaultAgentService::new().with_memory_manager(mm);
    }

    agent.initialize(&config);

    XLog::i(TAG, &format!("Executing task: {}", &prompt[..prompt.len().min(100)]));
    let callback = DefaultAgentCallback;
    agent.execute_task(&prompt, &callback);
    agent.shutdown();
}
