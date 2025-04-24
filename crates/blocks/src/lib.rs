// Terminal block components for VoidCLI
//
// This module provides reusable UI components for terminal interfaces

/// Represents a UI block in the terminal
#[derive(Debug, Clone)]
pub struct Block {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    title: Option<String>,
}

impl Block {
    /// Creates a new block with the specified dimensions
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Block {
            x,
            y,
            width,
            height,
            title: None,
        }
    }

    /// Sets the title of the block
    pub fn with_title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    /// Returns the dimensions of the block
    pub fn dimensions(&self) -> (u16, u16, u16, u16) {
        (self.x, self.y, self.width, self.height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_creation() {
        let block = Block::new(0, 0, 10, 5);
        assert_eq!(block.dimensions(), (0, 0, 10, 5));
    }

    #[test]
    fn test_block_with_title() {
        let block = Block::new(0, 0, 10, 5).with_title("Test Block");
        assert_eq!(block.title, Some("Test Block".to_string()));
    }
}

