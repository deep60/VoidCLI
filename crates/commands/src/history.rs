use serde::{Deserialize, Serialize}

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
            exit_code: None,
            duration_ms: None,
        }
    }
}
