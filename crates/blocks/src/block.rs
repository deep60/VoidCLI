use crate::command::Command;
use chrono::{DateTime, Utc};
use ropey::Rope;
use serde::{Deserialize, Serialize};
use std::process::Output;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub id: usize,
    pub command: Command,
    pub output: Output,
    pub created_at: DateTime<Utc>,
    pub exit_code: Option<i32>,
    pub duration_ms: Option<u64>,
    pub is_pinned: bool,
    pub is_folded: bool,
}

impl Block {
    pub fn new(id: usize, command: Command) -> Self {
        Self {
            id,
            command,
            output: Output::new(),
            created_at: Utc::now(),
            exit_code: None,
            duration_ms: None,
            is_pinned: false,
            is_folded: false,
        }
    }
}
