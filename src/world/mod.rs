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

struct CoreSystems {
    voxel: VoxelSystem,
    render: Option<RenderSystem>,
}

struct World {
    core_systems: CoreSystems,
}

/// Allows for the incremental initialization of a `World` object
struct WorldBuilder {
    voxel_system: Option<VoxelSystem>,
    render_system: Option<RenderSystem>
}

impl WorldBuilder {
    pub fn new() -> WorldBuilder {
        WorldBuilder {
            voxel_system: None,
            render_system: None,
        }
    }

    pub fn attach_voxel_system(&mut self, system: VoxelSystem) {
        self.voxel_system = Some(system);
    }

    pub fn attach_render_system(&mut self, system: RenderSystem) {
        self.render_system = Some(system);
    }

    pub fn init_world(self) -> Result<World, ()> {
        let mut world = World {
            core_systems: CoreSystems {
                voxel: self.voxel_system.ok_or(())?,
                render: self.render_system,
            }
        };

        Ok(world)
    }
}

impl World {
    pub fn render(&self) {
        self.core_systems.render.unwrap().render();
    }

    pub fn update(&mut self) {
        
    }
}