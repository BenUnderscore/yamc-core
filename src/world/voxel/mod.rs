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
use array::VoxelArray;
use thiserror::Error;

//Modules
mod array;
mod registry;

//Exports
pub use registry::{VoxelAttributeRegistry, VoxelNameRegistry};

#[derive(Error, Debug)]
pub enum VoxelError {
    #[error("An attribute has already been registered for ID {0}")]
    AttributeAlreadyRegistered(u16),
    #[error("An attribute that has been requested is missing for ID {0}")]
    AttributeMissing(u16),
    #[error("A name has already been registered with the ID {0}")]
    NameAlreadyRegistered(u16),
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

pub struct VoxelSystem {
    chunks: ChunkArray<VoxelArray>,
}

impl VoxelSystem {
    pub fn new() -> VoxelSystem {
        VoxelSystem {
            chunks: ChunkArray::new(),
        }
    }

    pub fn get_chunk(&self, x: i32, y: i32, z: i32) -> Option<&VoxelArray> {
        self.chunks.get(x, y, z)
    }

    pub fn get_chunk_mut(&mut self, x: i32, y: i32, z: i32) -> Option<&mut VoxelArray> {
        self.chunks.get_mut(x, y, z)
    }
}
