use anyhow::Result;
use log::info;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

use config::Config;
use crate::events::{Event, EventLoop};
use crate::state::AppState;

// Terminal implementation using alacritty_terminal
pub struct Terminal {
    // Add terminal fields here
}

impl Terminal {
    pub fn new(_config: &Config, _event_tx: mpsc::Sender<Event>) -> Self {
        // This is a placeholder implementation
        Self {}
    }

    pub async fn initialize(&self) -> Result<()> {
        Ok(())
    }
}

// Placeholder for renderer
pub struct Renderer {
    _config: Config,
}

impl Renderer {
    pub fn new(config: &Config) -> Self {
        Self { _config: config.clone() }
    }

    pub async fn initialize(&self) -> Result<()> {
        Ok(())
    }
}

// Placeholder for block manager
pub struct BlockManager {
    _state: Arc<Mutex<AppState>>,
}

impl BlockManager {
    pub fn new(state: Arc<Mutex<AppState>>) -> Self {
        Self { _state: state }
    }
}

pub struct VoidCLI {
    _config: Config,
    _state: Arc<Mutex<AppState>>,
    terminal: Terminal,
    renderer: Renderer,
    _block_manager: BlockManager,
    event_loop: EventLoop,
}

impl VoidCLI {
    pub fn new(config: Config) -> Self {
        let state = Arc::new(Mutex::new(AppState::new()));
        let (event_tx, event_rx) = mpsc::channel(100);
        let terminal = Terminal::new(&config, event_tx.clone());
        let renderer = Renderer::new(&config);
        let block_manager = BlockManager::new(state.clone());
        let event_loop = EventLoop::new(state.clone(), event_rx);

        Self {
            _config: config,
            _state: state,
            terminal,
            renderer,
            _block_manager: block_manager,
            event_loop,
        }
    }

    pub async fn run(&self) -> Result<()> {
        info!("Initializing application components");

        //Initializing the renderer
        self.renderer.initialize().await?;

        //initialize the terminal
        self.terminal.initialize().await?;

        //start the event loop
        self.event_loop.run().await?;

        Ok(())
    }
}
