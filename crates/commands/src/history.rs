use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct History {
    /// Path to history file
    history_file: PathBuf,
    entries: Vec<HistoryEntry>,
    max_entries: usize,
    position: usize,
}

impl History {
    pub fn new() -> Self {
        let home_dir = dirs::home_dir().unwrap_or_default();
        let history_file = home_dir.join(".void_history");

        let mut history = Self {
            history_file,
            entries: Vec::new(),
            max_entries: 1000,
            position: 0,
        };

        let _ = history.load();
        history
    }

    pub fn with_file<P: AsRef<Path>>(path: P) -> Self {
        let mut history = Self {
            history_file: path.as_ref().to_path_buf(),
            entries: Vec::new(),
            max_entries: 1000,
            position: 0,
        };
        let _ = history.load();
        history
    }

    pub fn load(&mut self) -> Result<()> {
        if !self.history_file.exists() {
            return Ok(());
        }

        let file = File::open(&self.history_file).context("Failed to open history file")?;
        let reader = BufReader::new(file);

        self.entries.clear();

        for line in reader.lines() {
            let line = line.context("Failed to read history line")?;
            if line.is_empty() {
                continue;
            }

            if let Ok(entry) = serde_json::from_str(&line) {
                self.entries.push(entry);
            }
        }

        self.position = self.entries.len();
        Ok(())
    }

    pub fn save(&self) -> Result<()> {
        let mut file = File::create(&self.history_file).context("Failed to create history file")?;

        for entry in &self.entries {
            let line = serde_json::to_string(entry).context("Failed to serialize history entry")?;
            writeln!(file, "{}", line).context("Failed to write history entry")?;
        }

        Ok(())
    }

    pub fn add(&mut self, entry: HistoryEntry) {
        if entry.command.trim().is_empty() {
            return;
        }

        if let Some(last) = self.entries.last() {
            if last.command == entry.command {
                return;
            }
        }

        self.entries.push(entry);

        if self.entries.len() > self.max_entries {
            let to_remove = self.entries.len() - self.max_entries;
            self.entries.drain(0..to_remove);
        }

        self.position = self.entries.len();

        let _ = self.save();
    }

    pub fn up(&mut self) -> Option<&HistoryEntry> {
        if self.position > 0 {
            self.position -= 1;
            self.entries.get(self.position)
        } else {
            None
        }
    }

    pub fn down(&mut self) -> Option<&HistoryEntry> {
        if self.position < self.entries.len() - 1 {
            self.position += 1;
            self.entries.get(self.position)
        } else {
            None
        }
    }

    pub fn search(&self, query: &str) -> Vec<&HistoryEntry> {
        let query = query.to_lowercase();
        self.entries
            .iter()
            .filter(|entry| entry.command.to_lowercase().contains(&query))
            .collect()
    }

    pub fn entries(&self) -> &[HistoryEntry] {
        &self.entries
    }

    pub fn clear(&mut self) -> Result<()> {
        self.entries.clear();
        self.position = 0;
        self.save()
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

    pub fn with_exit_code(mut self, exit_code: i32) -> Self {
        self.exit_code = Some(exit_code);
        self
    }

    pub fn with_duration(mut self, duration_ms: u64) -> Self {
        self.duration_ms = Some(duration_ms);
        self
    }
}
