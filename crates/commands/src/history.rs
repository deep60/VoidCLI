use serde::{Deserialize, Serialize};
use std::fs::File;

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
    }
}
