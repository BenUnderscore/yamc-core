//! This module has utilities for converting between different coordinate frames.
//!
//! In the YAMC engine there are three types of voxel coordinates:
//! - Chunk-local voxel coordinates (Local coordinates) (`u32`)
//! - Global chunk coordinates (Chunk coordinates) (`i32`)
//! - Global voxel coordinates (Global coordinates) (`i32`)
//!
//! Local coordinates index voxels within a given chunk and are relative to its origin.
//! They are used to directly index into the voxel buffer, but are useless outside of the context of a chunk.
//!
//! Chunks can be indexed with chunk coordinates which specify a given chunk within the world.
//! These can be used in combination with local coordinates to specify any voxel within the world.
//!
//! Because this can be cumbersome and a higher level abstraction is required for certain gameplay logic,
//! global coordinates are simply voxel coordinates that are relative to the world origin.
//!
//! When passing coordinates to functions, they should be passed with all components being separate.
//! This makes it easier to do immediate math on the numbers, since they do not need to be
//! accessed with a dot operator every time.
//! When there are multiple sets of coordinates passed to a function, it is okay to pass them as tuples
//! and to call them `xyz_*`

//Uses
use super::chunk;

/// Takes a combination of local coordinates and chunk coordinates and returns the global coordinates for it
pub fn local_to_global(xyz_local: (u32, u32, u32), xyz_chunk: (i32, i32, i32)) -> (i32, i32, i32) {
    (
        xyz_local.0 as i32 + xyz_chunk.0 * chunk::CHUNK_SIZE_X as i32,
        xyz_local.1 as i32 + xyz_chunk.1 * chunk::CHUNK_SIZE_X as i32,
        xyz_local.2 as i32 + xyz_chunk.2 * chunk::CHUNK_SIZE_X as i32
    )
}

pub fn global_to_local(x: i32, y: i32, z: i32) -> ((u32, u32, u32), (i32, i32, i32)) {
    let chunk_coords = (
        x.div_euclid(chunk::CHUNK_SIZE_X as i32),
        y.div_euclid(chunk::CHUNK_SIZE_Y as i32),
        z.div_euclid(chunk::CHUNK_SIZE_Z as i32)
    );

    let local_coords = (
        x.rem_euclid(chunk::CHUNK_SIZE_X as i32) as u32,
        y.div_euclid(chunk::CHUNK_SIZE_Y as i32) as u32,
        z.div_euclid(chunk::CHUNK_SIZE_Z as i32) as u32
    );

    (local_coords, chunk_coords)
}