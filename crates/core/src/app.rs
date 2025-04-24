use anyhow::Result;
use log::info;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

use VoidCLI_config::Config;

use crate::events::EventLoop;
use crate::state::AppState;

pub struct VoidCLI {
    config: Config,
    state: Arc<Mutex<AppState>>,
    terminal: Terminal,
    renderer: Renderer,
    block_manager: BlockManager,
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
            config,
            state,
            terminal,
            renderer,
            block_manager,
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
