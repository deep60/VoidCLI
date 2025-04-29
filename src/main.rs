use clap::{Parser, command};
use log::info;
use anyhow::Result;
use config::Config;
use core::app::VoidCLI;

#[derive(Parser)]
#[command()]
struct Cli {
    config: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    info!("Starting VoidCLI Terminal");

    let config = match cli.config {
        Some(ref path) => Config::from_file(path)?,
        None => Config::default(),
    };

    let app = VoidCLI::new(config);
    app.run().await?;

    info!("Shutting down");
    Ok(())
}
