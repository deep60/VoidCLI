use std::{
    process::Stdio,
    sync::mpsc,
    os::unix::io::{AsRawFd, OwnedFd, FromRawFd},
};

use anyhow::{Context, Result};
use tokio::{
    process::Command as TokioCommand,
    io::{AsyncReadExt, AsyncWriteExt},
    sync::oneshot,
};
use log::info;

use crate::{TermEvent, pty::PtyPair};

// manages a terminal process
pub struct ProcessManager {
    /// The child process
    child: Option<tokio::process::Child>,
    /// The shell command to run
    shell: String,
    /// Event Sender for process events
    event_sender: mpsc::Sender<TermEvent>,
    /// Working directory
    working_directory: String,
    ///Environment variables
    env_vars: Vec<(String, String)>,
}

impl ProcessManager {
    // Create a new process manager
    pub fn new(
        shell: &str,
        event_sender: mpsc::Sender<TermEvent>,
        working_directory: Option<&str>,
        env_vars: Vec<(String, String)>,
    ) -> Self {
        let working_directory = working_directory.map(|s| s.to_string()).unwrap_or_else(|| {
            std::env::current_dir()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|_| "/".to_string())
        });

        Self {
            child: None,
            shell: shell.to_string(),
            event_sender,
            working_directory,
            env_vars,
        }
    }

    /// Spawn a new process
    pub async fn spawn(&mut self) -> Result<()> {
        // Create a pseudo-terminal
        let pty = PtyPair::new()?;

        // Set up the command
        let mut command = TokioCommand::new(&self.shell);
        command.current_dir(&self.working_directory);

        // Add environment variables
        for (key, value) in &self.env_vars {
            command.env(key, value);
        }

        // Standard environment variables
        command.env("TERM", "xterm-256color");

        // Connect the command to our pty
        #[cfg(unix)]
        {
            let slave_fd = unsafe { OwnedFd::from_raw_fd(pty.slave.as_raw_fd()) };
            command.stdin(Stdio::from(slave_fd.try_clone()?));
            command.stdout(Stdio::from(slave_fd.try_clone()?));
            command.stderr(Stdio::from(slave_fd));
        }

        #[cfg(windows)]
        {
            // Windows implementation would be different
            // This is a placeholder
            command.stdin(Stdio::piped());
            command.stdout(Stdio::piped());
            command.stderr(Stdio::piped());
        }

        // Spawn the process
        let mut child = command.spawn().context("Failed to spawn process")?;

        // Set up output handling
        let mut master = pty.master;
        let event_sender = self.event_sender.clone();

        // Create a channel for process status
        let (status_tx, status_rx) = oneshot::channel();

        // Store the child process first
        self.child = Some(child);

        // Spawn a task to handle process output
        tokio::spawn(async move {
            let mut buffer = vec![0u8; 4096];

            loop {
                match master.read(&mut buffer).await {
                    Ok(0) => {
                        // EOF - process has terminated
                        break;
                    }
                    Ok(n) => {
                        // Send the output to the event handler
                        let output_data = buffer[0..n].to_vec();
                        if let Err(_) = event_sender.send(TermEvent::Output(output_data)) {
                            break;
                        }
                    }
                    Err(e) => {
                        let error_msg = format!("Error reading from process: {}", e);
                        let _ = event_sender.send(TermEvent::Error(error_msg));
                        break;
                    }
                }
            }

            // Process has terminated
            let _ = status_tx.send(());
        });

        // Wait for the process to exit in the background
        tokio::spawn(async move {
            if let Ok(()) = status_rx.await {
                info!("Process output stream closed");
            }
        });

        Ok(())
    }

    /// Write data to the process
    pub async fn write(&mut self, data: &[u8]) -> Result<()> {
        if let Some(child) = &mut self.child {
            if let Some(stdin) = &mut child.stdin {
                stdin.write_all(data).await?;
                stdin.flush().await?;
            }
        }

        Ok(())
    }

    // Resize the terminal
    pub async fn resize(&mut self, _cols: u16, _rows: u16) -> Result<()> {
        // This would use the PTY resize functionality
        // A placeholder for now
        #[cfg(unix)]
        if let Some(_) = &mut self.child {
            // Here we would use the winsize struct from libc
        }

        Ok(())
    }

    /// kill the process
    pub async fn kill(&mut self) -> Result<()> {
        if let Some(child) = &mut self.child {
            match child.try_wait() {
                Ok(None) => {
                    // Still running, kill it
                    child.kill().await?;
                    Ok(())
                }
                Ok(Some(status)) => {
                    // Already exited
                    let code = status.code().unwrap_or(-1);
                    let _ = self.event_sender.send(TermEvent::ProcessExit(code));
                    Ok(())
                }
                Err(e) => Err(e.into()),
            }
        } else {
            Ok(())
        }
    }

    // Set working directory
    pub fn set_working_directory(&mut self, dir: &str) {
        self.working_directory = dir.to_string();
    }

    /// Add environment variables
    pub fn add_env_var(&mut self, key: &str, value: &str) {
        self.env_vars.push((key.to_string(), value.to_string()));
    }
}
