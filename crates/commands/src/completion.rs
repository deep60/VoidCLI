use core::str;
use std::{
    collections::HashSet,
    fmt::format,
    fs,
    io::Empty,
    os::{darwin::fs, unix::fs::PermissionsExt},
    path::PathBuf,
};

use anyhow::{Context, Ok};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Completion {
    // Add fields as needed
    system_paths: Vec<PathBuf>,
    command_cahe: HashSet<String>,
    cache_initialized: bool,
}

impl Completion {
    pub fn new() -> Self {
        let system_paths = if let Ok(path_var) = std::env::var("PATH") {
            std::env::split_paths(&path_var).collect()
        } else {
            Vec::new()
        };

        Self {
            system_paths,
            command_cahe: HashSet::new(),
            cache_initialized: false,
        }
    }

    pub fn initialize_cache(&mut self) -> Result<()> {
        // Placeholder implementation
        if self.cache_initialized {
            return Ok(());
        }

        for path in &self.system_paths {
            if path.exists() && path.is_dir() {
                let entries = fs::read_dir(path)
                    .with_context(|| format!("Failed to read directory: {}", path.display()))?;

                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();

                        if path.is_dir() {
                            continue;
                        }

                        #[cfg(unix)]
                        let executable = fs::metadata(&path)
                            .map(|meta| meta.permissions().mode() & 0o111 != 0)
                            .unwrap_or(false);

                        #[cfg(not(unix))]
                        let executable = path
                            .extension()
                            .map_or(false, |ext| ext == "exe" || ext == "bat" || ext == "cmd");

                        if executable {
                            if let Some(name) = path.file_name() {
                                if let Some(name_str) = name.to_str() {
                                    self.command_cahe.insert(name_str.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }

        self.cache_initialized = true;
        Ok(())
    }

    pub fn complete_command(&mut self, partial: &str) -> Vec<String> {
        if !self.cache_initialized {
            let _ = self.initialize_cache();
        }

        self.command_cahe
            .iter()
            .filter(|cmd| cmd.starts_with(partial))
            .cloned()
            .collect()
    }

    pub fn complete_path(&self, partial: &str) -> Vec<String> {
        let mut results = Vec::new();

        let (dir_path, prefix) = if partial.contains('/') || partial.contains('\\') {
            let path = PathBuf::from(partial);
            if let Some(parent) = path.parent() {
                let prefix = path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("");
                (parent.to_path_buf(), prefix.to_string())
            } else {
                (PathBuf::from("."), partial.to_string())
            }
        } else {
            (PathBuf::from("."), partial.to_string())
        };

        if let Ok(entries) = fs::read_dir(&dir_path) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.starts_with(&prefix) {
                        let mut path = if dir_path == PathBuf::from(".") {
                            name.to_string()
                        } else {
                            format!("{}/{}", dir_path.display(), name)
                        };

                        if entry.path().is_dir() {
                            path.push('/');
                        }

                        results.push(path);
                    }
                }
            }
        }

        results
    }

    pub fn complete(&mut self, line: &str, cursor_pos: usize) -> Vec<String> {
        if line.is_empty() {
            return Vec::new();
        }

        let (before_cursor, _) = line.split_at(cursor_pos);
        let tokens: Vec<&str> = before_cursor.split_whitespace().collect();

        if tokens.is_empty() {
            return Vec::new();
        }

        if tokens.len() == 1 {
            return self.complete_command(tokens[0]);
        }

        let partial = tokens.last().unwrap();
        self.complete_path(partial)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_completion_initialization() {
        let mut completion = Completion::new();
        assert!(!completion.cache_initialized);

        let result = completion.initialize_cache();
        assert!(result.is_ok());
        assert!(completion.cache_initialized);
    }
}
