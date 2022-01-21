//! This module has utilities for converting between different coordinate frames.
//!
//! In the YAMC engine there are three types of voxel coordinates:
//! - Chunk-local voxel coordinates (Local coordinates)
//! - Global chunk coordinates (Chunk coordinates)
//! - Global voxel coordinates (Global coordinates)
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
//! All coordinates should use signed 32-bit numbers (`i32`) since they are in the sweet spot of being large
//! (4 billion x 4 billion voxels for global coordinates) and small.
//!
//! When passing coordinates to functions, they should be passed with all components being separate.
//! This makes it easier to do immediate math on the numbers, since they do not need to be
//! accessed with a dot operator every time.
//! When there are multiple sets of coordinates passed to a function, it is okay to pass them as tuples
//! and to call them `xyz_*`

pub fn local_to_global(xyz_local: (i32, i32, i32), xyz_chunk: (i32, i32, i32)) -> (i32, i32, i32) {
    unimplemented!()
}
