use cgmath::Vector3;
use crate::creation::chunk::Chunk;
use crate::game_specs::{CHUNK_SIZE, NUM_CHUNKS};

pub struct World {
    pub chunks : Vec<Chunk>
}

impl World {
    pub fn new() -> Self {
        let mut chunks = Vec::new();

        for x in 0..NUM_CHUNKS {
            for z in 0..NUM_CHUNKS {
                chunks.push(Chunk::generate(Vector3::new(x as f32, -1.0, z as f32)));
            }
        }

        World {
            chunks
        }
    }
}