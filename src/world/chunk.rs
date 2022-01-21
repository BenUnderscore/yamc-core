//Uses
use super::voxel::Voxel;

pub const CHUNK_SIZE_X: usize = 32;
pub const CHUNK_SIZE_Y: usize = 32;
pub const CHUNK_SIZE_Z: usize = 32;

pub struct VoxelArray {
    array: Box<[Voxel]>,
}

impl VoxelArray {
    pub fn new(default_voxel: Voxel) -> VoxelArray {
        VoxelArray {
            array: vec![default_voxel; CHUNK_SIZE_X * CHUNK_SIZE_Y * CHUNK_SIZE_Z]
                .into_boxed_slice(),
        }
    }

    pub fn get_voxel_at_position(&self, x: usize, y: usize, z: usize) -> &Voxel {
        &self.array[VoxelArray::get_voxel_index(x, y, z)]
    }

    pub fn get_voxel_at_position_mut(&mut self, x: usize, y: usize, z: usize) -> &mut Voxel {
        &mut self.array[VoxelArray::get_voxel_index(x, y, z)]
    }

    pub fn get_voxel_at_index(&self, i: usize) -> &Voxel {
        &self.array[i]
    }

    pub fn get_voxel_at_index_mut(&mut self, i: usize) -> &mut Voxel {
        &mut self.array[i]
    }

    pub fn get_voxel_index(x: usize, y: usize, z: usize) -> usize {
        z * (CHUNK_SIZE_X * CHUNK_SIZE_Y) + y * CHUNK_SIZE_X + x
    }
}

pub struct Chunk {
    voxels: VoxelArray,
}

impl Chunk {
    pub fn new(default_voxel: Voxel) -> Chunk {
        Chunk {
            voxels: VoxelArray::new(default_voxel),
        }
    }

    pub fn get_voxels(&self) -> &VoxelArray {
        &self.voxels
    }

    pub fn get_voxels_mut(&mut self) -> &mut VoxelArray {
        &mut self.voxels
    }
}
