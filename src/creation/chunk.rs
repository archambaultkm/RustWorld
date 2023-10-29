use std::char::MAX;
use cgmath::Vector3;
use crate::creation::cube::Cube;
use crate::creation::cube::CubeType::{AIR, GRASS, STONE};
use crate::game_specs::{CHUNK_SIZE, MAX_CHUNK_HEIGHT};

pub struct Chunk {
    position : Vector3<f32>,
    pub cubes : Vec<Cube>,
}

impl Chunk {
    pub fn generate(position : Vector3<f32>) -> Self {

        // TODO add noise stuff here

        let mut cubes = Vec::new();

        for x in 0..CHUNK_SIZE + 1 {
            for y in 0..MAX_CHUNK_HEIGHT + 1 {
                for z in 0..CHUNK_SIZE + 1 {
                    cubes.push(Cube::new(Vector3::new(
                        position.x * CHUNK_SIZE as f32 + x as f32,
                        position.y * MAX_CHUNK_HEIGHT as f32 + y as f32,
                        position.z * CHUNK_SIZE as f32 + z as f32
                    ), AIR)); //todo figure out how you're doing block types
                }
            }
        }

        Chunk {
            position,
            cubes
        }
    }

    pub fn has_cube(&self, cube_position : Vector3<f32>) -> bool {
        //todo super inefficient
        return self.cubes.iter().any( |cube| cube.position == cube_position);
    }
}
