// Command handling module for VoidCLI
//
// This module provides functionality for parsing and executing terminal commands

/// Represents a terminal command
#[derive(Debug, Clone)]
pub struct Command {
    name: String,
    args: Vec<String>,
}

impl Command {
    /// Creates a new command with the given name
    pub fn new(name: &str) -> Self {
        Command {
            name: name.to_string(),
            args: Vec::new(),
        }
    }

    /// Adds arguments to the command
    pub fn with_args(mut self, args: Vec<&str>) -> Self {
        self.args = args.iter().map(|&s| s.to_string()).collect();
        self
    }

    /// Parse a command string into a Command object
    pub fn parse(input: &str) -> Option<Self> {
        let mut parts = input.trim().split_whitespace();
        
        if let Some(name) = parts.next() {
            let args: Vec<String> = parts.map(String::from).collect();
            Some(Command {
                name: name.to_string(),
                args,
            })
        } else {
            None
        }
    }

    /// Returns the command name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the command arguments
    pub fn args(&self) -> &[String] {
        &self.args
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_creation() {
        let cmd = Command::new("ls").with_args(vec!["-l", "-a"]);
        assert_eq!(cmd.name(), "ls");
        assert_eq!(cmd.args(), &["-l", "-a"]);
    }

    #[test]
    fn test_command_parsing() {
        let cmd = Command::parse("git commit -m 'Initial commit'").unwrap();
        assert_eq!(cmd.name(), "git");
        assert_eq!(cmd.args()[0], "commit");
        assert_eq!(cmd.args()[1], "-m");
        assert_eq!(cmd.args()[2], "'Initial");
        assert_eq!(cmd.args()[3], "commit'");
    }
}

