use anyhow::Result;
use crate::memory::store::MemoryStore;
use crate::utils::logger::XLog;
use serde::{Deserialize, Serialize};

const TAG: &str = "MemoryManager";

pub struct MemoryManager {
    store: MemoryStore,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub topic: String,
    pub content: String,
    pub timestamp: i64,
}

impl MemoryManager {
    pub fn new(db_path: &str) -> Result<Self> {
        let store = MemoryStore::new(db_path)?;
        XLog::i(TAG, &format!("MemoryManager initialized at {}", db_path));
        Ok(Self { store })
    }

    pub fn save(&self, topic: &str, content: &str) -> Result<()> {
        let entry = MemoryEntry {
            topic: topic.to_string(),
            content: content.to_string(),
            timestamp: chrono::Utc::now().timestamp(),
        };
        let json = serde_json::to_string(&entry)?;
        self.store.insert(&format!("mem:{}", topic), &json)?;
        XLog::d(TAG, &format!("Saved memory: {}", topic));
        Ok(())
    }

    pub fn load(&self, topic: &str) -> Result<Option<MemoryEntry>> {
        let val = self.store.get(&format!("mem:{}", topic))?;
        match val {
            Some(json) => Ok(Some(serde_json::from_str(&json)?)),
            None => Ok(None),
        }
    }

    pub fn search(&self, query: &str) -> Vec<MemoryEntry> {
        let records = self.store.search_by_prefix("mem:");
        let q = query.to_lowercase();
        records.into_iter()
            .filter_map(|r| serde_json::from_str::<MemoryEntry>(&r.value).ok())
            .filter(|e| e.topic.to_lowercase().contains(&q) || e.content.to_lowercase().contains(&q))
            .collect()
    }
}
