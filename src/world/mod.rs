//Modules
pub mod chunk;
pub mod coords;
pub mod voxel;

//Uses
use crate::render::RenderSystem;
use voxel::VoxelSystem;

struct CoreSystems {
    voxel: VoxelSystem,
    render: Option<RenderSystem>,
}

struct World {
    core_systems: CoreSystems,
}