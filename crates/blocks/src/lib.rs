// Terminal block components for VoidCLI
//
// This module provides reusable UI components for terminal interfaces

/// Represents a UI block in the terminal
mod block;
mod command;
mod navigation;
mod output;

use std::sync::Arc;
use tokio::sync::Mutex;

pub use block::Block;

/// Block manager that stores and manages terminal UI blocks
pub struct BlockManager<A> {
    state: Arc<Mutex<A>>,
    blocks: Vec<Block>,
}

impl<A> BlockManager<A> {
    pub fn new(state: Arc<Mutex<A>>) -> Self {
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
