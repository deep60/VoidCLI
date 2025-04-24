// Terminal functionality module for VoidCLI
//
// This module handles terminal emulation, PTY handling, and terminal state management.

/// Represents a terminal instance
#[derive(Debug)]
pub struct Terminal {
    // Terminal properties will be added here
    width: u16,
    height: u16,
}

impl Terminal {
    /// Creates a new terminal with default dimensions
    pub fn new() -> Self {
        Terminal {
            width: 80,
            height: 24,
        }
    }

    /// Creates a new terminal with specified dimensions
    pub fn with_size(width: u16, height: u16) -> Self {
        Terminal { width, height }
    }

    /// Returns the current terminal dimensions
    pub fn size(&self) -> (u16, u16) {
        (self.width, self.height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terminal_creation() {
        let term = Terminal::new();
        assert_eq!(term.size(), (80, 24));
    }
}

