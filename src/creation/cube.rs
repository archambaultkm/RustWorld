use cgmath::{vec3, Vector3};
use rand::Rng;
use crate::creation::chunk::Chunk;

// each cube type is also assigned a number for passing to the fragment shader in chunk.rs
#[derive(Copy, Clone, PartialEq)]
pub enum CubeType {
    AIR = 0,
    GRASS = 1,
    DIRT = 2,
    STONE = 3
}

impl CubeType {
    pub(crate) fn random() -> CubeType {
        // Generate a random number between 0 and 3 for the existing cube types
        let random_number = rand::thread_rng().gen_range(0, 4);

        // Match the random number to a variant
        match random_number {
            0 => CubeType::AIR,
            1 => CubeType::GRASS,
            2 => CubeType::DIRT,
            3 => CubeType::STONE,
            _ => unreachable!(), // This should never happen
        }
    }
}

#[derive(Clone)]
pub struct Cube {
    pub _type : CubeType,
    pub vertices : [f32; 180],
    pub indices: [u32; 36], // 6 faces * 2 triangles * 3 vertices
    pub position : Vector3<f32>
}

impl Cube {
    pub(crate) const VERTICES: [f32; 180] = [
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
    ];

    pub(crate) const INDICES: [u32; 36] = [
        0, 1, 2, 2, 3, 0, // Front Face
        4, 5, 6, 6, 7, 4, // Back Face
        8, 9, 10, 10, 11, 8, // Left Face
        12, 13, 14, 14, 15, 12, // Right Face
        16, 17, 18, 18, 19, 16, // Bottom Face
        20, 21, 22, 22, 23, 20, // Top Face
    ];

    pub fn new(position : Vector3<f32>, _type : CubeType) -> Self {
        let mut cube = Cube {
            position,
            _type,
            vertices : Cube::VERTICES, // Initialize with zeros
            indices: Cube::INDICES,
        };

        cube
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
            if chunk.at(neighbor) != CubeType::AIR {
                num_neighbors += 1;
            }
        }

        //only return true if it's fully "occluded"
        if num_neighbors == 6 {return true};

        return false;
    }

    pub fn set_type(&mut self, type_ : CubeType) {
        self._type = type_;
    }
}
pub fn determine_cube_type(noise_value: f64, position: Vector3<f32>, y: usize) -> CubeType {
    // Guarantee "bedrock" layer of stone
    if y == 0 {
        return CubeType::STONE;
    }

    // TODO adjust to be more interesting
    if noise_value > 0.0 {
        if position.y < -5.0 {
            CubeType::STONE
        } else if position.y < 0.0 {
            CubeType::DIRT
        } else {
            CubeType::GRASS
        }

    } else {
        CubeType::AIR
    }
}
