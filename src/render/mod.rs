//Uses
use crate::event_loop::EventLoopProxy;
use crate::res::{LoadedResourceData, ResourceLoadType, ResourceSystem};
use bytemuck;
use pollster::block_on;
use std::cell::Cell;
use std::sync::Arc;
use wgpu;
use wgpu::util::DeviceExt;

//Module definitions
mod voxel;
mod surface;

//Exports
use voxel::VoxelRenderSystem;

pub struct RenderSystem {
    //WGPU resources
    device: Option<wgpu::Device>,
    queue: wgpu::Queue,

    //Subsystems
    voxel_system: VoxelRenderSystem,
}

impl RenderSystem {
    pub fn new(event_loop_proxy: &EventLoopProxy, res: &mut ResourceSystem) -> RenderSystem {
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
    }
}

// impl RenderContext {
//     pub fn init() -> Arc<RenderContext> {
//         let instance_tmp = wgpu::Instance::new(wgpu::Backends::all());
//         let (instance, surface_result) = event_loop_proxy.create_wgpu_surface(instance_tmp);
//         let surface = surface_result.unwrap();
//         let window_inner_size = event_loop_proxy.get_window_inner_size().unwrap();

//         let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
//             power_preference: wgpu::PowerPreference::default(),
//             compatible_surface: Some(&surface),
//             force_fallback_adapter: false,
//         }))
//         .unwrap();

//         let (device, queue) = block_on(adapter.request_device(
//             &wgpu::DeviceDescriptor {
//                 features: wgpu::Features::empty(),
//                 limits: wgpu::Limits::default(),
//                 label: None,
//             },
//             None,
//         ))
//         .unwrap();

//         let surface_config = wgpu::SurfaceConfiguration {
//             usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
//             format: surface.get_preferred_format(&adapter).unwrap(),
//             width: window_inner_size.width,
//             height: window_inner_size.height,
//             present_mode: wgpu::PresentMode::Fifo,
//         };
//         surface.configure(&device, &surface_config);

//         Arc::new(RenderContext {
//             device,
//             queue,
//             surface,
//             surface_format: surface_config.format,
//         })
//     }

//     fn get_device(&self) -> &wgpu::Device {
//         &self.device
//     }

//     fn get_queue(&self) -> &wgpu::Queue {
//         &self.queue
//     }

//     fn get_surface(&self) -> &wgpu::Surface {
//         &self.surface
//     }

//     fn get_surface_format(&self) -> wgpu::TextureFormat {
//         self.surface_format
//     }

//     fn begin_frame<'a>(self: Arc<RenderContext>) -> FrameRenderState<'a> {
//         let output = self.surface.get_current_texture().unwrap();
//         let view = output
//             .texture
//             .create_view(&wgpu::TextureViewDescriptor::default());

//         let mut command_encoder = self
//             .device
//             .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

//         let render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
//             label: None,
//             color_attachments: &[wgpu::RenderPassColorAttachment {
//                 view: &view,
//                 resolve_target: None,
//                 ops: wgpu::Operations {
//                     load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
//                     store: true,
//                 },
//             }],
//             depth_stencil_attachment: None,
//         });

//         FrameRenderState {
//             context: self,
//             command_encoder,
//             render_pass,
//         }
//     }
// }

// impl<'a> FrameRenderState<'a> {
//     pub fn end_frame(self) {
//         self.context
//             .queue
//             .submit(std::iter::once(self.encoder.finish()));
//         self.context.surface.present();
//     }
// }
