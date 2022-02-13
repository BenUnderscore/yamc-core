//Uses
use super::Renderer;
use crate::res;
use crate::world::chunk::ChunkArray;
use crate::world::voxel::VoxelAttributeRegistry;
use wgpu;

pub struct VoxelSolidModel {
    color: (f32, f32, f32),
}

pub enum VoxelAppearanceAttribute {
    /// A regular solid block,
    Solid(VoxelSolidModel),
    /// Completely transparent (air)
    None,
}

struct VoxelRenderChunkData {
    buffer: wgpu::Buffer,
}

pub struct VoxelRenderSystem {
    appearances: VoxelAttributeRegistry<VoxelAppearanceAttribute>,
    
    //WGPU stuff
    chunks: ChunkArray<VoxelRenderChunkData>,
    pipeline: wgpu::RenderPipeline,
}

impl VoxelRenderSystem {
    pub fn init(
        device: &wgpu::Device,
        resource_system: &res::ResourceSystem,
        appearances: VoxelAttributeRegistry<VoxelAppearanceAttribute>,
    ) -> VoxelRenderSystem {
        
    }
}

fn create_render_pipeline(
    device: &wgpu::Device,
    resource_system: &res::ResourceSystem
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
        push_constant_ranges: &[]
    });

    let vertex_buffer_layout = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<VoxelVertex>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &[
            wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x3,
            }
        ]
    };

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Voxel rendering pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vertex_main",
            buffers: &[
                vertex_buffer_layout
            ],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fragment_main",
            targets: &[wgpu::ColorTargetState {
                
            }];
        })
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct VoxelVertex {
    position: [f32; 3],
    color: [f32; 3]
}