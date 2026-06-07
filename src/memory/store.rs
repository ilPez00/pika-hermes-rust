use sled::Db;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryRecord {
    pub key: String,
    pub value: String,
    pub timestamp: i64,
}

pub struct MemoryStore {
    db: Db,
}

impl MemoryStore {
    pub fn new(path: &str) -> Result<Self> {
        let db = sled::open(path)?;
        Ok(Self { db })
    }

    pub fn insert(&self, key: &str, value: &str) -> Result<()> {
        let record = MemoryRecord {
            key: key.to_string(),
            value: value.to_string(),
            timestamp: chrono::Utc::now().timestamp(),
        };
        let json = serde_json::to_string(&record)?;
        self.db.insert(key.as_bytes(), json.as_bytes())?;
        self.db.flush()?;
        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<Option<String>> {
        Ok(self.db.get(key.as_bytes())?
            .map(|v| String::from_utf8_lossy(&v).to_string()))
    }

    pub fn delete(&self, key: &str) -> Result<()> {
        self.db.remove(key.as_bytes())?;
        Ok(())
    }

    pub fn search_by_prefix(&self, prefix: &str) -> Vec<MemoryRecord> {
        let mut results = Vec::new();
        for entry in self.db.scan_prefix(prefix.as_bytes()) {
            if let Ok((_, value)) = entry {
                if let Ok(record) = serde_json::from_slice::<MemoryRecord>(&value) {
                    results.push(record);
                }
            }
        }
        results
    }
}
