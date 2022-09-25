
use trait_set::trait_set;
use bytemuck::Pod;
use wgpu;

pub const BLOCK_SIZE: u64 = 4096;

fn allocate_buffer(device: &wgpu::Device, blocks: u64) -> wgpu::Buffer {
    let desc = wgpu::BufferDescriptor {
        label: None,
        size: blocks * BLOCK_SIZE,
        usage: wgpu::BufferUsages::MAP_WRITE | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    };

    device.create_buffer(&desc)
}

fn calculate_minimum_block_count(bytes: u64) -> u64 {
    let blocks = bytes / BLOCK_SIZE;
    let remainder = bytes - (blocks * BLOCK_SIZE);
    if remainder > 0 { blocks + 1 } else { blocks }
}

pub struct VertexBuffer {
    buffer: wgpu::Buffer,
    allocated_blocks: u64,
    used_bytes: u64,
}

trait_set! {
    pub trait Vertex = Pod;
}

impl VertexBuffer {
    pub fn new(device: &wgpu::Device) -> VertexBuffer {
        let buffer = allocate_buffer(device, 1);

        VertexBuffer {
            buffer: buffer,
            allocated_blocks: 1,
            used_bytes: 0
        }
    }

    pub fn reallocate_vertices<V: Vertex>(&mut self, device: &wgpu::Device, count: u64) -> bool {
        let target_byte_count = count * std::mem::size_of::<V>() as u64;
        let target_block_count = calculate_minimum_block_count(target_byte_count);
        
        let reallocate = self.allocated_blocks < target_block_count;
        
        if reallocate {
            self.buffer = allocate_buffer(device, target_block_count);
            self.allocated_blocks = target_block_count;
        }

        self.used_bytes = target_byte_count;
        reallocate
    }

    pub fn update<V: Vertex>(&mut self) -> BufferUpdate<V> {
        let binding_resource = self.buffer.as_entire_binding();
    }
}