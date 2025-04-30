use serde::{Deserialize, Serialize};
use std::process;

/// Representation of command output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Output {
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub status: Option<i32>,
}

impl Output {
    /// Create a new empty Output instance
    pub fn new() -> Self {
        Self {
            stdout: Vec::new(),
            stderr: Vec::new(),
            status: None,
        }
    }

    /// Append stdout content
    pub fn append_stdout(&mut self, data: &[u8]) {
        self.stdout.extend_from_slice(data);
    }

    /// Append stderr content
    pub fn append_stderr(&mut self, data: &[u8]) {
        self.stderr.extend_from_slice(data);
    }

    /// Set exit status
    pub fn set_status(&mut self, status: i32) {
        self.status = Some(status);
    }

    /// Get stdout as string, handling UTF-8 conversion
    pub fn stdout_string(&self) -> String {
        String::from_utf8_lossy(&self.stdout).into_owned()
    }

    /// Get stderr as string, handling UTF-8 conversion
    pub fn stderr_string(&self) -> String {
        String::from_utf8_lossy(&self.stderr).into_owned()
    }

    /// Was the command successful
    pub fn success(&self) -> bool {
        self.status == Some(0)
    }
}

impl From<process::Output> for Output {
    fn from(output: process::Output) -> Self {
        Self {
            stdout: output.stdout,
            stderr: output.stderr,
            status: output.status.code(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output() {
        let mut output = Output::new();
        output.append_stdout(b"Hello, world");
        output.append_stderr(b"Error message");
        output.set_status(0);

        assert_eq!(output.stdout_string(), "Hello, world");
        assert_eq!(output.stderr_string(), "Error message");
        assert!(output.success());
    }
}
