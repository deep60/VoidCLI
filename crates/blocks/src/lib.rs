// Terminal block components for VoidCLI
//
// This module provides reusable UI components for terminal interfaces

/// Represents a UI block in the terminal
mod block;
mod command;
mod navigation;
mod output;

use std::{io::Seek, sync::Arc};
use tokio::sync::Mutex;

pub use block::Block;

pub struct BlockManager {
    state: Arc<Mutex<AppState>>,
    blocks: Vec<Block>,
}

impl BlockManager {
    pub fn new(state: Arc<Mutex<AppState>>) -> Self {
        Self {
            state,
            blocks: Vec::new(),
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn get_block(&self, id: usize) -> Option<&Block> {
        self.blocks.get(id)
    }

    pub fn get_current_block(&self) -> Option<&Block> {
        self.blocks.last()
    }
}
