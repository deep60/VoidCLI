use anyhow::{Context, Result};
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

    pub async fn initialize(&mut self, window: &'a Window) -> Result<()> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = instance.create_surface(window)
            .context("Failed to create surface")?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .context("Failed to find an appropriate adapter")?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .context("Failed to create device")?;

        self.device = Some(device);
        self.queue = Some(queue);
        self.surface = Some(surface);

        Ok(())
    }

    pub fn render(&mut self) -> Result<()> {
        if let (Some(device), Some(queue), Some(surface)) = (
            &self.device,
            &self.queue,
            &self.surface,
        ) {
            let frame = surface
                .get_current_texture()
                .context("Failed to acquire next swap chain texture")?;
            let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

            {
                let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None,
                });
            }

            queue.submit(std::iter::once(encoder.finish()));
            frame.present();
        }

        Ok(())
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if let (Some(device), Some(surface)) = (&self.device, &self.surface) {
            let format = surface.get_capabilities(device).formats[0];

            surface.configure(
                device,
                &wgpu::SurfaceConfiguration {
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                    format,
                    width,
                    height,
                    present_mode: wgpu::PresentMode::Fifo,
                    alpha_mode: wgpu::CompositeAlphaMode::Auto,
                    view_formats: vec![],
                    desired_maximum_frame_latency: 2,
                },
            );
        }
    }
}
