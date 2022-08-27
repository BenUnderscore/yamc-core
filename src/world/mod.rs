//! Contains all simulation fundamentals
//!
//! The `World` object contains an entire simulation context.
//! The world consists of a collection of systems.
//! These systems are each responsible for one aspect of the simulation.

//Modules
pub mod chunk;
pub mod coords;
pub mod voxel;

//Uses
use crate::render::RenderSystem;
use voxel::VoxelSystem;

pub struct CoreSystems {
    pub voxel: VoxelSystem,
    pub render: Option<RenderSystem>,
}