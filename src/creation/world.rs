use cgmath::Vector3;
use rand::Rng;
use crate::creation::chunk::Chunk;
use crate::game_specs::{CHUNK_SIZE, NUM_CHUNKS};

pub struct World {
    pub chunks : Vec<Chunk>,
    pub world_seed : u32
}

impl World {
    pub fn new() -> Self {
        let mut chunks = Vec::new();
        let world_seed = rand::thread_rng().gen();  // Generate a random seed for the entire world
        let mut previous_chunk : Option<Chunk> = None;

        // TODO chunks should eventually be loaded in a radius from player position
        for x in 0..NUM_CHUNKS {
            for z in 0..NUM_CHUNKS {
                let new_chunk_position = Vector3::new(x as f32, -1.0, z as f32);
                let new_chunk = Chunk::generate(new_chunk_position, previous_chunk.as_ref(), world_seed);

                chunks.push(new_chunk);

                // Update previous_chunk to Some(new_chunk) for the next iteration
                previous_chunk = Some(chunks.last().unwrap().clone());
            }
        }

        World {
            chunks,
            world_seed
        }
    }

    fn get_chunk_at(&self, position: Vector3<f32>) -> Option<&Chunk> {
        // Find and return the chunk at the specified position
        self.chunks.iter().find(|&chunk| chunk.position == position)
    }

    pub fn generate_chunk(&mut self, position: Vector3<f32>) {
        if !self.chunk_exists(position) {
            // Check if the neighboring chunks exist
            let neighbor_chunk_x = self.get_chunk_at(position + Vector3::new(CHUNK_SIZE as f32, 0.0, 0.0));
            let neighbor_chunk_z = self.get_chunk_at(position + Vector3::new(0.0, 0.0, CHUNK_SIZE as f32));

            let chunk = Chunk::generate(position, neighbor_chunk_z, self.world_seed);
            self.chunks.push(chunk);
        }
    }

    fn chunk_exists(&self, position: Vector3<f32>) -> bool {
        self.chunks.iter().any(|chunk| chunk.position == position)
    }
}