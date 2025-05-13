use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use anyhow::Result;

use crate::state::AppState;

pub enum Event {
    // Define your events here
    Quit,
}

pub struct EventLoop {
    _state: Arc<Mutex<AppState>>,
    _event_rx: mpsc::Receiver<Event>,
}

impl EventLoop {
    pub fn new(state: Arc<Mutex<AppState>>, event_rx: mpsc::Receiver<Event>) -> Self {
        Self { _state: state, _event_rx: event_rx }
    }

    pub async fn run(&self) -> Result<()> {
        // Placeholder implementation
        Ok(())
    }
}

