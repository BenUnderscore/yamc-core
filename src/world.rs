//Modules
pub mod chunk;
pub mod coords;
pub mod voxel;

//Uses
use chunk::Chunk;

pub trait ChunkGenerator {
    fn generate(&self, x: i32, y: i32, z: i32) -> Chunk;
}

pub struct World {
    test_chunk: Chunk,
    chunk_generator: Box<dyn ChunkGenerator>,
}

impl World {
    pub fn new(chunk_generator: Box<dyn ChunkGenerator>) -> World {
        World {
            test_chunk: chunk_generator.generate(0, 0, 0),
            chunk_generator: chunk_generator,
        }
    }

    pub fn get_chunk_at_position(&self, x_chunk: i32, y_chunk: i32, z_chunk: i32) -> &Chunk {
        assert_eq!((x_chunk, y_chunk, z_chunk), (0, 0, 0));
        &self.test_chunk
    }
}