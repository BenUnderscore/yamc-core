//! This module contains definitions for voxels and voxel-related structures
//!
//! The `Voxel` struct is meant to be the representation of a voxel in a grid
//!
//! Voxels' behavior is defined through a collection of _attributes_.
//! These are type objects that are stored in `VoxelAttributeRegistry` instances
//! and can be looked up to determine the particular behavior of a voxel.
//! Voxels can for example have attributes for appearance, hardness, mining drops, etc.

//Uses
use super::chunk::ChunkArray;
use std::sync::Arc;
use thiserror;

//Modules
mod array;
mod registry;

//Exports
pub use array::VoxelArray;
pub use registry::{Attribute, AttributeRegistries, AttributeRegistry, NameRegistry};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("An attribute has already been registered for ID {0}")]
    AttributeAlreadyRegistered(u16),
    #[error("An attribute that has been requested is missing for ID {0}")]
    AttributeMissing(u16),
    #[error("A name has already been registered with the ID {0}")]
    NameAlreadyRegistered(u16),
    #[error("An attribute registry has already been added! Attribute name: {0}")]
    RegistryAlreadyAdded(&'static str),
    #[error("The chunk at ({0}, {1}, {2}) has already been loaded!")]
    ChunkAlreadyLoaded(i32, i32, i32),
}

/// One block in a chunk
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Voxel {
    /// Represents the type of this voxel
    pub id: u16,
    /// Metadata associated with this voxel
    pub data: u16,
}

impl Voxel {
    pub fn id_eq(&self, other: &Voxel) -> bool {
        self.id == other.id
    }
}

pub enum Event {
    ChunkLoaded {
        coords_x: i32,
        coords_y: i32,
        coords_z: i32,
    },
}

pub struct VoxelSystem {
    chunks: ChunkArray<VoxelArray>,
    name_registry: NameRegistry,
    attribute_registries: registry::AttributeRegistries,
    recorded_events: Vec<Event>,
}

impl VoxelSystem {
    pub fn new(
        name_registry: NameRegistry,
        attribute_registries: registry::AttributeRegistries,
    ) -> VoxelSystem {
        VoxelSystem {
            chunks: ChunkArray::new(),
            name_registry,
            attribute_registries,
            recorded_events: Vec::new(),
        }
    }

    pub fn get_chunk(&self, x: i32, y: i32, z: i32) -> Option<&VoxelArray> {
        self.chunks.get(x, y, z)
    }

    pub fn name_registry(&self) -> &NameRegistry {
        &self.name_registry
    }

    pub fn get_attribute_registry<A: Attribute>(&self) -> Option<Arc<AttributeRegistry<A>>> {
        self.attribute_registries.get_registry::<A>()
    }

    pub fn reset_events(&mut self) {
        self.recorded_events.clear();
    }

    pub fn get_events(&self) -> &Vec<Event> {
        &self.recorded_events
    }

    pub fn load_chunk(&mut self, voxels: VoxelArray, x: i32, y: i32, z: i32) -> Result<(), Error> {
        self.chunks
            .try_add(voxels, x, y, z)
            .map_err(|_| Error::ChunkAlreadyLoaded(x, y, z))?;
        self.recorded_events.push(Event::ChunkLoaded {
            coords_x: x,
            coords_y: y,
            coords_z: z,
        });

        Ok(())
    }
}
