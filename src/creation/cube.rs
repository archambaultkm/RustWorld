use cgmath::{vec3, Vector3};

// each cube type is also assigned a number for passing to the fragment shader in chunk.rs
#[derive(Copy, Clone)]
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
