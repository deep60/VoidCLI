use anyhow::Result;
use config::Config;
use themes::Theme;
use wgpu::{Device, Queue, Surface};
use winit::window::Window;

pub struct Renderer<'a> {
    config: Config,
    device: Option<Device>,
    queue: Option<Queue>,
    surface: Option<Surface<'a>>,
    theme: Theme,
}

impl<'a> Renderer<'a> {
    pub fn new(config: Config, theme: Theme) -> Self {
        Self {
            config,
            device: None,
            queue: None,
            surface: None,
            theme,
        }
    }

    pub async fn initialize(&mut self, window: &Window) -> Result<(), wgpu::Error> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = unsafe { instance.create_surface(window) }?;
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await?;

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
            },
            None,
        ).await?;

        self.device = Some(device);
        self.queue = Some(queue);
        self.surface = Some(surface);

        Ok(())
    }

    pub fn render(&mut self) -> Result<(), wgpu::Error> {
        if let (Some(device), Some(queue), Some(surface)) = (
            &self.device,
            &self.queue,
            &self.surface,
        ) {
            let frame = surface.get_current_texture()?;
            let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            }),
                            store: true,
                        },
                    })],
                    depth_stencil_attachment: None,
                });
            }

            queue.submit(std::iter::once(encoder.finish()));
            frame.present();
        }

        Ok(())
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if let (Some(device), Some(surface)) = (&self.device, &self.surface) {
            surface.configure(
                device,
                &wgpu::SurfaceConfiguration {
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                    format: surface.get_supported_formats(device)[0],
                    width,
                    height,
                    present_mode: wgpu::PresentMode::Fifo,
                    alpha_mode: wgpu::CompositeAlphaMode::Auto,
                    view_formats: vec![],
                },
            );
        }
    }
}
