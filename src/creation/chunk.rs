use std::arch::x86_64::_rdseed16_step;
use cgmath::{Vector2, Vector3};
use noise::{NoiseFn, OpenSimplex, Seedable};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use crate::creation::cube::{Cube, CubeType};
use crate::game_specs::{CHUNK_SIZE, MAX_CHUNK_HEIGHT};

pub struct Chunk {
    position : Vector3<f32>,
    pub cubes : Vec<Cube>,
}

impl Chunk {
    pub fn generate(position : Vector3<f32>) -> Self {

        // Generate a random seed
        let seed = rand::thread_rng().gen();

        // make a new noise generator based on the seed
        let perlin = OpenSimplex::new(seed);

        let mut cubes = Vec::new();

        for x in 0..CHUNK_SIZE + 1 {
            for y in 0..MAX_CHUNK_HEIGHT + 1 {
                for z in 0..CHUNK_SIZE + 1 {

                    let x_noise = (position.x * CHUNK_SIZE as f32 + x as f32) / 50.0;
                    let y_noise = (position.y * MAX_CHUNK_HEIGHT as f32 + y as f32) / 50.0;
                    let z_noise = (position.z * CHUNK_SIZE as f32 + z as f32) / 50.0;

                    let noise_value = perlin.get([x_noise as f64, y_noise as f64, z_noise as f64]);

                    println!("{}", noise_value); // TODO for debugging

                    let cube_type = determine_cube_type(noise_value);

                    cubes.push(Cube::new(Vector3::new(
                        position.x * CHUNK_SIZE as f32 + x as f32,
                        position.y * MAX_CHUNK_HEIGHT as f32 + y as f32,
                        position.z * CHUNK_SIZE as f32 + z as f32
                    ), cube_type));
                }
            }
        }

        Chunk {
            position,
            cubes
        }
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
        //self.cubes[]
    }
}

//todo move to cube probably
fn determine_cube_type(noise_value: f64) -> CubeType {
    // TODO adjust
    if noise_value <= 0.25 && noise_value > 0.15 {
        CubeType::GRASS
    } else if noise_value > 0.0 && noise_value < 0.15 {
        CubeType::DIRT
    } else if noise_value < 0.0 {
        CubeType::STONE
    } else {
       CubeType::AIR
    }
}
