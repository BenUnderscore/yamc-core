//Modules
pub mod chunk;
pub mod coords;
pub mod voxel;

//Uses
use voxel::VoxelSystem;

pub trait WorldSystems {
    fn get_voxel_system(& self) -> &VoxelSystem;
}