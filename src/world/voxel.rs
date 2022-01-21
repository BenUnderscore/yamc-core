//! This module contains definitions for voxels and voxel-related structures
//! The `Voxel` struct is meant to be the representation of a voxel in a grid
//!
//! Voxels' behavior is defined through a collection of _attributes_.
//! These are type objects that are stored in `VoxelAttributeRegistry` instances
//! and can be looked up to determine the particular behavior of a voxel.
//! Voxels can for example have attributes for appearance, hardness, mining drops, etc.

use std::collections::HashMap;
use thiserror::Error;

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

/// Stores one type of attribute for all registered voxel types in a cache-friendly way
pub struct VoxelAttributeRegistry<A> {
    map: Vec<Option<A>>,
    label: String,
}

impl<A> VoxelAttributeRegistry<A> {
    pub fn new(attribute_label: &str) -> VoxelAttributeRegistry<A> {
        VoxelAttributeRegistry {
            map: Vec::new(),
            label: attribute_label.to_owned(),
        }
    }

    pub fn get_label(&self) -> &str {
        &self.label
    }

    pub fn register(&mut self, id: u16, attribute_obj: A) -> Result<(), VoxelError> {
        if id as usize >= self.map.len() {
            self.map.resize_with(id as usize, Option::default);
        }

        match self.map[id as usize] {
            Some(_) => Err(VoxelError::AttributeAlreadyRegistered(id)),
            None => Ok(self.map[id as usize] = Some(attribute_obj)),
        }
    }

    pub fn find(&self, id: u16) -> Result<&A, VoxelError> {
        if id as usize >= self.map.len() {
            return Err(VoxelError::AttributeMissing(id));
        }
        match &self.map[id as usize] {
            Some(attr) => Ok(attr),
            None => Err(VoxelError::AttributeMissing(id)),
        }
    }
}

/// Allows for a reverse-lookup of strings to voxel IDs, useful for scripting convenience
/// and serialization consistency.
pub struct VoxelNameRegistry {
    map: HashMap<String, u16>,
}

impl VoxelNameRegistry {
    pub fn new() -> VoxelNameRegistry {
        VoxelNameRegistry {
            map: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: &str, id: u16) -> Result<(), VoxelError> {
        match self.map.insert(name.to_owned(), id) {
            Some(id) => Err(VoxelError::NameAlreadyRegistered(id)),
            None => Ok(()),
        }
    }

    pub fn find(&self, name: &str) -> Option<u16> {
        self.map.get(name).map(|id| *id)
    }
}
