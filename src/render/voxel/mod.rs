//Uses
use super::Camera;
use crate::res;
use crate::world::chunk::ChunkArray;
use crate::world::voxel;
use crate::world::voxel::VoxelSystem;
use log::{debug, trace};

use wgpu;
use wgpu::util::DeviceExt;

//Modules
mod mesh;

//Exports
pub use mesh::{AppearanceAttribute, SolidColorCubeModel};

struct ChunkData {
    buffer: wgpu::Buffer,
    vertex_count: u64,
}

pub(super) struct VoxelRenderSystem {
    //Chunk array
    chunks: ChunkArray<ChunkData>,

    //WGPU resources
    pipeline: wgpu::RenderPipeline,
}

pub(super) struct PipelineInitParams {
    pub output_texture_format: wgpu::TextureFormat,
}

impl VoxelRenderSystem {
    pub(super) fn new(
        res: &mut res::ResourceSystem,
        device: &wgpu::Device,
        pipeline_init: PipelineInitParams,
    ) -> VoxelRenderSystem {
        let pipeline = create_render_pipeline(device, res, &pipeline_init);

        VoxelRenderSystem {
            chunks: ChunkArray::new(),
            pipeline,
        }
    }

    pub fn update(&mut self, voxel_system: &VoxelSystem, device: &wgpu::Device, queue: &wgpu::Queue) {
        let appearance_registry = voxel_system
            .get_attribute_registry::<AppearanceAttribute>()
            .unwrap();

        let voxel_events = voxel_system.get_events();
        for ev in voxel_events.iter() {
            match ev {
                voxel::Event::ChunkLoaded {
                    coords_x,
                    coords_y,
                    coords_z,
                } => {
                    let voxel_array =
                        voxel_system.get_chunk(*coords_x, *coords_y, *coords_z).unwrap();
                    let mesh = mesh::generate_mesh(voxel_array, appearance_registry.as_ref());
                    let buffer = device.create_buffer_init(
                        &wgpu::util::BufferInitDescriptor {
                            label: Some("Voxel mesh"),
                            contents: bytemuck::cast_slice(&mesh[..]),
                            usage: wgpu::BufferUsages::VERTEX
                        }
                    );

                    let chunk_data = ChunkData {
                        buffer: buffer,
                        vertex_count: mesh.len() as u64
                    };
                    self.chunks.add(chunk_data, *coords_x, *coords_y, *coords_z);
                    trace!("Chunk loaded at coordinates: {:?}", (*coords_x, *coords_y, *coords_z))
                }
                _ => (),
            }
        }
    }

    pub(super) fn encode_commands(
        &self,
        device: &wgpu::Device,
        color_buf: wgpu::TextureView,
        camera: &Camera,
    ) -> wgpu::CommandBuffer {
        let mut command_encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("VoxelRenderSystem"),
        });

        {
            let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Voxel rendering"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &color_buf,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.pipeline);
            for (coords, chunk_data) in self.chunks.iter() {
                render_pass.set_vertex_buffer(0, chunk_data.buffer.slice(..));
                render_pass.draw(0..chunk_data.vertex_count as u32, 0..1);
            }
        }

        command_encoder.finish()
    }
}

fn create_render_pipeline(
    device: &wgpu::Device,
    resource_system: &mut res::ResourceSystem,
    pipeline_init: &PipelineInitParams,
) -> wgpu::RenderPipeline {
    let shader_source_res = resource_system
        .get_loaded_resource("shaders/voxel.wgsl", res::ResourceLoadType::PlainText)
        .unwrap();
    let shader_source = shader_source_res.data.as_text().unwrap();
    let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
        label: Some("shaders/voxel.wgsl"),
        source: wgpu::ShaderSource::Wgsl(shader_source.into()),
    });
    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });
    let vertex_buffer_layout = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<mesh::Vertex>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &mesh::Vertex::vertex_attribute_array(0, 1)
    };
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Voxel rendering pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[vertex_buffer_layout],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[wgpu::ColorTargetState {
                format: pipeline_init.output_texture_format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            }],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    })
}
