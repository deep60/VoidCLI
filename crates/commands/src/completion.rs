use std::{
    fs,
    os::unix::fs::PermissionsExt,
    path::PathBuf,
};

use anyhow::Result;

pub struct CommandCompletion {
    cache: Vec<String>,
    system_paths: Vec<PathBuf>,
}

impl CommandCompletion {
    pub fn new() -> Self {
        Self {
            cache: Vec::new(),
            system_paths: Vec::new(),
        }
    }

    pub fn initialize_cache(&mut self) -> Result<()> {
        // Get system paths from PATH environment variable
        if let Some(path_var) = std::env::var("PATH").ok() {
            self.system_paths = path_var
                .split(':')
                .map(PathBuf::from)
                .collect();
        }

        // Scan system paths for executables
        for path in &self.system_paths {
            if let Some(entries) = fs::read_dir(path).ok() {
                for entry in entries {
                    if let Some(entry) = entry.ok() {
                        if let Some(metadata) = entry.metadata().ok() {
                            if metadata.is_file() && metadata.permissions().mode() & 0o111 != 0 {
                                if let Some(name) = entry.file_name().to_str() {
                                    self.cache.push(name.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub fn get_completions(&self, prefix: &str) -> Vec<String> {
        self.cache
            .iter()
            .filter(|cmd| cmd.starts_with(prefix))
            .cloned()
            .collect()
    }

    pub fn scan_directory(&self, dir_path: &PathBuf) -> Vec<String> {
        let mut completions = Vec::new();

        if let Some(entries) = fs::read_dir(dir_path).ok() {
            for entry in entries {
                if let Some(entry) = entry.ok() {
                    if let Some(metadata) = entry.metadata().ok() {
                        if metadata.is_file() && metadata.permissions().mode() & 0o111 != 0 {
                            if let Some(name) = entry.file_name().to_str() {
                                completions.push(name.to_string());
                            }
                        }
                    }
                }
            }
        }

        completions
    }
}

#[derive(Debug, Clone)]
pub struct Completion {
    system_paths: Vec<PathBuf>,
    command_cache: Vec<String>,
    cache_initialized: bool,
}

impl Completion {
    pub fn new() -> Self {
        let system_paths = if let Some(path_var) = std::env::var("PATH").ok() {
            std::env::split_paths(&path_var).collect()
        } else {
            Vec::new()
        };

        Self {
            system_paths,
            command_cache: Vec::new(),
            cache_initialized: false,
        }
    }

    pub fn initialize_cache(&mut self) -> Result<()> {
        if self.cache_initialized {
            return Ok(());
        }

        for path in &self.system_paths {
            if path.exists() && path.is_dir() {
                if let Some(entries) = fs::read_dir(path).ok() {
                    for entry in entries {
                        if let Some(entry) = entry.ok() {
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
                                        self.command_cache.push(name_str.to_string());
                                    }
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

        self.command_cache
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

        if let Some(entries) = fs::read_dir(&dir_path).ok() {
            for entry in entries {
                if let Some(entry) = entry.ok() {
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
