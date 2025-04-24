use anyhow::Result;
use config::Config;
use theme::Theme;
use wgpu::{Device, Queue, Surface, SurfaceConfiguration};
use winit::window::Window;

pub struct Renderer {
    config: Config,
    device: Option<Device>,
    queue: Option<Queue>,
    surface: Option<Surface>,
    surface_config: Option<SurfaceConfiguration>,
    theme: Theme,
}

impl Renderer {
    pub fn new(config: &Config) -> Self {
        let theme = Theme::from_name(&config.theme).unwrap_or_default();

        Self {
            config: config.clone(),
            device: None,
            queue: None,
            surface: None,
            surface_config: None,
            theme,
        }
    }

    pub fn initialize(&self) -> Result<()> {
        //initialize WGPU, create device, queue, etc;
        Ok(())
    }
}
