use cgmath::{Vector3};
use noise::{NoiseFn, OpenSimplex};
use crate::creation::cube::{Cube, CubeType, determine_cube_type};
use crate::creation::noise::{get_layered_noise, Noise};
use crate::game_specs::{CHUNK_SIZE, MAX_CHUNK_HEIGHT, MIN_CHUNK_HEIGHT};

#[derive(Clone)]
pub struct Chunk {
    pub position : Vector3<f32>,
    pub cubes : Vec<Cube>,
}

impl Chunk {
    pub fn new(position : Vector3<f32>, cubes : Vec<Cube>) -> Self {
        Chunk {
            position,
            cubes
        }
    }

    pub fn generate(position: Vector3<f32>, neighbor_chunk: Option<&Chunk>, world_seed : u32) -> Self {
        // Make a new noise generator based on the seed
        let noise = Noise::new(world_seed);

        let mut cubes = Vec::new();

        for x in 0..CHUNK_SIZE + 1 {
            for y in 0..MAX_CHUNK_HEIGHT + 1 {
                for z in 0..CHUNK_SIZE + 1 {
                    // Original cube position without adjustments
                    let cube_position = Vector3::new(
                        position.x * CHUNK_SIZE as f32 + x as f32,
                        position.y * MAX_CHUNK_HEIGHT as f32 + y as f32,
                        position.z * CHUNK_SIZE as f32 + z as f32,
                    );

                    // Adjust position based on neighbor chunk
                    let adjusted_position = match neighbor_chunk {
                        Some(neighbor) if x == 0 && neighbor.position.x < position.x => {
                            Vector3::new(cube_position.x - CHUNK_SIZE as f32, cube_position.y, cube_position.z)
                        }
                        Some(neighbor) if z == 0 && neighbor.position.z < position.z => {
                            Vector3::new(cube_position.x, cube_position.y, cube_position.z - CHUNK_SIZE as f32)
                        }
                        _ => cube_position,
                    };

                    let generated_noise = get_layered_noise(
                        noise.get_base_noise(adjusted_position),
                        0.5,
                        noise.get_detail_noise(adjusted_position),
                        0.5
                    );

                    let cube_type = determine_cube_type(generated_noise, adjusted_position, y);

                    cubes.push(Cube::new(adjusted_position, cube_type));
                }
            }
        }

        Chunk::new(position, cubes)
    }

    // check if this chunk contains a cube at the given position
    pub fn has_cube(&self, cube_position : Vector3<f32>) -> bool {
        //todo super inefficient
        return self.cubes.iter().any( |cube| cube.position == cube_position);
    }

    // get cube type at given position
    pub fn at(&self, cube_position : Vector3<f32>) -> CubeType {
        let cube = self.cubes.iter().find(|cube| cube_position == cube.position);

        return cube.unwrap()._type;
    }

    // set cube
    pub fn set(&self, position : Vector3<f32>, _type : CubeType) {
        let mut cube = self.cubes.iter().find(|cube| position == cube.position);
        //cube.set_type(_type); todo error
    }
}


