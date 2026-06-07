use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysisConfig {
    pub base_url: String,
}

impl Default for PhysisConfig {
    fn default() -> Self {
        Self { base_url: "http://127.0.0.1:19876".to_string() }
    }
}

pub struct PhysisClient {
    config: PhysisConfig,
    client: reqwest::Client,
}

impl PhysisClient {
    pub fn new(config: PhysisConfig) -> Self {
        Self { config, client: reqwest::Client::new() }
    }

    pub fn new_default() -> Self {
        Self::new(PhysisConfig::default())
    }

    pub fn is_running_http() -> bool {
        std::net::TcpStream::connect("127.0.0.1:19876").is_ok()
    }

    pub async fn ingest(&self, goal: &str, prompt: &str) -> Result<String, String> {
        let url = format!("{}/api/v1/ingest", self.config.base_url);
        let body = serde_json::json!({
            "goal": goal,
            "prompt": prompt,
        });

        match self.client.post(&url).json(&body).send().await {
            Ok(resp) => {
                let text = resp.text().await.unwrap_or_default();
                Ok(text)
            }
            Err(e) => Err(format!("Ingest failed: {}", e)),
        }
    }

    pub async fn query(&self, query: &str) -> Result<String, String> {
        let url = format!("{}/api/v1/wiki/query", self.config.base_url);
        let body = serde_json::json!({ "query": query });

        match self.client.post(&url).json(&body).send().await {
            Ok(resp) => {
                let text = resp.text().await.unwrap_or_default();
                Ok(text)
            }
            Err(e) => Err(format!("Query failed: {}", e)),
        }
    }
}
