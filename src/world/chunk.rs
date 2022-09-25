//! Defines the basics of chunks and some utilities
//!
//! The primary type of this module is `ChunkArray`, which is an array
//! that is designed to hold one object per chunk.

//Modules

//! Import with wildcard to bring in the CHUNK_SIZE_* constants
pub mod size {
    pub const CHUNK_SIZE_X: usize = 16;
    pub const CHUNK_SIZE_Y: usize = 16;
    pub const CHUNK_SIZE_Z: usize = 16;
}

//Uses
use size::*;
use std::collections::BTreeMap;

pub struct ChunkArray<T> {
    chunks: BTreeMap<(i32, i32, i32), T>,
}

impl<T> ChunkArray<T> {
    pub fn new() -> ChunkArray<T> {
        ChunkArray {
            chunks: BTreeMap::new(),
        }
    }

    pub fn get(&self, x: i32, y: i32, z: i32) -> Option<&T> {
        self.chunks.get(&(x, y, z))
    }

    pub fn get_mut(&mut self, x: i32, y: i32, z: i32) -> Option<&mut T> {
        self.chunks.get_mut(&(x, y, z))
    }

    pub fn add(&mut self, chunk: T, x: i32, y: i32, z: i32) {
        self.try_add(chunk, x, y, z).unwrap();
    }

    pub fn try_add(&mut self, chunk: T, x: i32, y: i32, z: i32) -> Result<(), ()> {
        let key = (x, y, z);
        if self.chunks.contains_key(&key) {
            Err(())
        } else {
            self.chunks.insert(key, chunk);
            Ok(())
        }
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter::new(self)
    }
}


#[derive(Clone)]
pub struct Iter<'a, T> {
    internal: std::collections::btree_map::Iter<'a, (i32, i32, i32), T>,
}

impl<'a, T> Iter<'a, T> {
    fn new(chunk_array: &'a ChunkArray<T>) -> Iter<'a, T> {
        Iter {
            internal: chunk_array.chunks.iter()
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (&'a (i32, i32, i32), &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.internal.next()
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.internal.len()
    }
}