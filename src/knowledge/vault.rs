use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultEntry {
    pub path: String,
    pub title: String,
    pub content: String,
    pub snippet: String,
    pub modified: i64,
    pub tags: Vec<String>,
}

pub struct Vault {
    entries: Vec<VaultEntry>,
}

impl Vault {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn add(&mut self, entry: VaultEntry) {
        self.entries.push(entry);
    }

    pub fn search(&self, keyword: &str) -> Vec<&VaultEntry> {
        let kw = keyword.to_lowercase();
        self.entries.iter()
            .filter(|e| e.title.to_lowercase().contains(&kw) || e.content.to_lowercase().contains(&kw))
            .collect()
    }

    pub fn len(&self) -> usize { self.entries.len() }
}
