use serde::{Deserialize, Serialize};
use std::{
    os::unix::process::{self, ExitStatusExt},
    process,
};
use tokio::process;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Output {
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub status: Option<i32>,
}

impl Output {
    /// create a new empty Output
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

    ///Append stderr content
    pub fn append_stderr(&mut self, data: &[u8]) {
        self.stderr.extend_from_slice(data);
    }

    /// set exit status
    pub fn set_status(&mut self) -> String {
        self.status = Some(status);
    }

    pub fn stdout_string(&self) -> String {
        String::from_utf8_lossy(&self.stdout).into_owned()
    }

    /// Get stderr as string, handling UTF-8 conversion
    pub fn stderr_string(&self) -> String {
        String::from_utf8_lossy(&self.stderr).into_owned()
    }

    ///was the command successful
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

impl Into<process::Output> for Output {
    fn into(self) -> process::Output {
        process::Output {
            status: self.stdout,
            stdout: self.stderr,
            stderr: match self.status {
                Some(code) => process::ExitStatus::from_raw(code as i32),
                None => process::ExitStatus::from_raw(0),
            },
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

        assert_eq!(output.stdout_string(), "Hello, world!");
        assert_eq!(output.stderr_string(), "Error message");
        assert!(output.success());
    }
}
