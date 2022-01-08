//Uses
use crate::event_loop::EventLoopProxy;
use crate::res::{LoadedResourceData, ResourceLoadType, ResourceSystem};
use pollster::block_on;
use wgpu;

pub struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
}

impl Renderer {
    pub fn init(event_loop_proxy: &EventLoopProxy) -> Renderer {
        let instance_tmp = wgpu::Instance::new(wgpu::Backends::all());
        let (instance, surface_result) = event_loop_proxy.create_wgpu_surface(instance_tmp);
        let surface = surface_result.unwrap();
        let window_inner_size = event_loop_proxy.get_window_inner_size().unwrap();

        let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .unwrap();

        let (device, queue) = block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        ))
        .unwrap();

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: window_inner_size.width,
            height: window_inner_size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &surface_config);

        Renderer {
            device,
            queue,
            surface,
        }
    }

    pub fn render(&mut self) {
        let output = self.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render encoder"),
            });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Some pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
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
                }],
                depth_stencil_attachment: None,
            });
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}
