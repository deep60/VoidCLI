use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct History {
    entries: Vec<HistoryEntry>,
    max_entries: usize,
}

impl History {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            max_entries: 1000,
        }
    }

    pub fn add(&mut self, command: &str) {
        if self.entries.len() >= self.max_entries {
            self.entries.remove(0);
        }
        self.entries.push(HistoryEntry::new(command));
    }

    pub fn get_entries(&self) -> &[HistoryEntry] {
        &self.entries
    }
}

///A command history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    ///The command string
    pub command: String,
    ///The timestamp when the command was excuted
    pub timestamp: u64,
    ///working directory when command was excuted
    pub working_dir: String,
    ///exit code of the command
    pub exit_code: Option<i32>,
    ///Duration of command execution in milliseconds
    pub duration_ms: Option<u64>,
}

impl HistoryEntry {
    pub fn new(command: &str) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let working_dir = std::env::current_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();

        Self {
            command: command.to_string(),
            timestamp,
            working_dir,
            exit_code: None,
            duration_ms: None,
        }
    }
}
