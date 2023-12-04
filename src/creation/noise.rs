use cgmath::Vector3;
use noise::{NoiseFn, OpenSimplex};

pub struct Noise {
    perlin : OpenSimplex
}

impl Noise {

    pub fn new(world_seed : u32) -> Noise {
        let perlin = OpenSimplex::new(world_seed);

        Noise {
            perlin
        }
    }
    pub fn get_base_noise(&self, cube_position : Vector3<f32>) -> f64 {
        self.perlin.get([
            (cube_position.x / 50.0) as f64,
            (cube_position.y / 40.0) as f64,
            (cube_position.z / 50.0) as f64,
        ])
    }

    pub fn get_detail_noise(&self, cube_position : Vector3<f32>) -> f64 {
        self.perlin.get([
            (cube_position.x / 10.0) as f64,
            (cube_position.y / 10.0) as f64,
            (cube_position.z / 10.0) as f64,
        ])
    }
}

pub fn get_layered_noise(base_noise : f64, base_noise_weight : f64, detail_noise : f64, detail_noise_weight : f64) -> f64 {
    (base_noise_weight * base_noise) + (detail_noise_weight * detail_noise)
}