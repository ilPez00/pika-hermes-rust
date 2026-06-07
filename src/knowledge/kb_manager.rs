use anyhow::Result;
use crate::knowledge::vault::{Vault, VaultEntry};
use crate::utils::logger::XLog;

const TAG: &str = "KBManager";

pub struct KBManager {
    vault: Vault,
}

impl KBManager {
    pub fn new() -> Self {
        XLog::i(TAG, "KBManager initialized");
        Self { vault: Vault::new() }
    }

    pub fn write(&mut self, path: &str, title: &str, content: &str) -> Result<()> {
        let snippet = if content.len() > 200 {
            format!("{}...", &content[..200])
        } else {
            content.to_string()
        };
        let entry = VaultEntry {
            path: path.to_string(),
            title: title.to_string(),
            content: content.to_string(),
            snippet,
            modified: chrono::Utc::now().timestamp(),
            tags: Vec::new(),
        };
        self.vault.add(entry);
        XLog::i(TAG, &format!("Wrote {} bytes to vault: {}", content.len(), path));
        Ok(())
    }

    pub fn search(&self, keyword: &str) -> Vec<&VaultEntry> {
        let results = self.vault.search(keyword);
        XLog::d(TAG, &format!("Search '{}': {} results", keyword, results.len()));
        results
    }

    pub fn read(&self, path: &str) -> Option<&VaultEntry> {
        self.vault.search(path).into_iter().next()
    }
}
