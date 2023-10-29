use cgmath::{vec3, Vector2, Vector3};
use crate::creation::chunk::Chunk;
use crate::game_specs::CHUNK_SIZE;

// each cube type is also assigned a number for passing to the fragment shader in chunk.rs
#[derive(Copy, Clone, PartialEq)]
pub enum CubeType {
    AIR = 0,
    GRASS = 1,
    DIRT = 2,
    STONE = 3
}

pub struct Cube {
    pub _type : CubeType,
    pub vertices : [f32; 180],
    pub position : Vector3<f32>
}

impl Cube {
    pub fn new(position : Vector3<f32>, _type : CubeType) -> Self {
        Cube {
            position,
            _type,
            ..Cube::default()
        }
    }

    fn direct_neighbors(&self) -> [Vector3<f32>; 6] {
        let mut neighbor_coords = [Vector3::new(0.0, 0.0, 0.0); 6];

        let i = self.position.x;
        let j = self.position.y;
        let k = self.position.z;

        neighbor_coords[0] = Vector3::new(i + 1.0, j, k); // Right
        neighbor_coords[1] = Vector3::new(i - 1.0, j, k); // Left
        neighbor_coords[2] = Vector3::new(i, j + 1.0, k); // Top
        neighbor_coords[3] = Vector3::new(i, j - 1.0, k); // Bottom
        neighbor_coords[4] = Vector3::new(i, j, k + 1.0); // Front
        neighbor_coords[5] = Vector3::new(i, j, k - 1.0); // Back

        neighbor_coords
    }

    pub fn is_blocked(&self, chunk : &Chunk) -> bool {

        let mut num_neighbors = 0;

        for neighbor in self.direct_neighbors() {
            if chunk.has_cube(neighbor) {
                num_neighbors += 1;
            }
        }

        //only return true if it's fully occluded
        if num_neighbors == 6 {return true};

        return false;
    }
}

impl Default for Cube {
    fn default() -> Self {
        Cube {
            position: vec3(0.0, 0.0, 0.0),
            _type : CubeType::AIR,

            vertices: [
                // Front Face
                0.5, -0.5, -0.5, 1.0, 1.0,
                0.5, 0.5, -0.5, 1.0, 0.0,
                -0.5, 0.5, -0.5, 0.0, 0.0,
                -0.5, 0.5, -0.5, 0.0, 0.0,
                -0.5, -0.5, -0.5, 0.0, 1.0,
                0.5, -0.5, -0.5, 1.0, 1.0,

                // Back Face
                0.5, -0.5, 0.5, 1.0, 1.0,
                0.5, 0.5, 0.5, 1.0, 0.0,
                -0.5, 0.5, 0.5, 0.0, 0.0,
                -0.5, 0.5, 0.5, 0.0, 0.0,
                -0.5, -0.5, 0.5, 0.0, 1.0,
                0.5, -0.5, 0.5, 1.0, 1.0,

                // Left Face
                -0.5, 0.5, -0.5, 1.0, 1.0,
                -0.5, -0.5, -0.5, 0.0, 1.0,
                -0.5, -0.5, 0.5, 0.0, 0.0,
                -0.5, -0.5, 0.5, 0.0, 0.0,
                -0.5, 0.5, 0.5, 1.0, 0.0,
                -0.5, 0.5, -0.5, 1.0, 1.0,

                // Right Face
                0.5, 0.5, -0.5, 1.0, 1.0,
                0.5, -0.5, -0.5, 0.0, 1.0,
                0.5, -0.5, 0.5, 0.0, 0.0,
                0.5, -0.5, 0.5, 0.0, 0.0,
                0.5, 0.5, 0.5, 1.0, 0.0,
                0.5, 0.5, -0.5, 1.0, 1.0,

                // Bottom Face
                -0.5, -0.5, -0.5, 0.0, 1.0,
                -0.5, -0.5, 0.5, 0.0, 0.0,
                0.5, -0.5, 0.5, 1.0, 0.0,
                0.5, -0.5, 0.5, 1.0, 0.0,
                0.5, -0.5, -0.5, 1.0, 1.0,
                -0.5, -0.5, -0.5, 0.0, 1.0,

                // Top Face
                -0.5, 0.5, -0.5, 0.0, 1.0,
                -0.5, 0.5, 0.5, 0.0, 0.0,
                0.5, 0.5, 0.5, 1.0, 0.0,
                0.5, 0.5, 0.5, 1.0, 0.0,
                0.5, 0.5, -0.5, 1.0, 1.0,
                -0.5, 0.5, -0.5, 0.0, 1.0,
            ]
        }
    }
}
