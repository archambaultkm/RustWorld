use cgmath::{vec3, Vector3};

pub struct Cube {
    pub vertices : [f32; 180],
    pub position : Vector3<f32>
}

impl Cube {
    pub fn new(position : Vector3<f32>, option: Option<()>) -> Self {
        Cube {
            position,
            ..Cube::default()
        }
    }

    //create a cube adjacent to an existing one in the specified direction
    pub fn from_existing(cube : Cube, direction : Vector3<f32>) -> Self {

        let position =  cube.position + direction;
        Cube {
            position,
            ..Cube::default()
        }
    }
}

impl Default for Cube {
    fn default() -> Self {
        Cube {
            position: vec3(0.0, 0.0, 0.0),

            //shows image right-side up
            vertices: [
                // Front Face
                -0.5, -0.5, -0.5, 0.0, 1.0,
                0.5, -0.5, -0.5, 1.0, 1.0,
                0.5, 0.5, -0.5, 1.0, 0.0,
                0.5, 0.5, -0.5, 1.0, 0.0,
                -0.5, 0.5, -0.5, 0.0, 0.0,
                -0.5, -0.5, -0.5, 0.0, 1.0,

                // Back Face
                -0.5, -0.5, 0.5, 0.0, 1.0,
                0.5, -0.5, 0.5, 1.0, 1.0,
                0.5, 0.5, 0.5, 1.0, 0.0,
                0.5, 0.5, 0.5, 1.0, 0.0,
                -0.5, 0.5, 0.5, 0.0, 0.0,
                -0.5, -0.5, 0.5, 0.0, 1.0,

                // Left Face
                -0.5, 0.5, 0.5, 1.0, 1.0,
                -0.5, 0.5, -0.5, 1.0, 0.0,
                -0.5, -0.5, -0.5, 0.0, 0.0,
                -0.5, -0.5, -0.5, 0.0, 0.0,
                -0.5, -0.5, 0.5, 0.0, 1.0,
                -0.5, 0.5, 0.5, 1.0, 1.0,

                // Right Face
                0.5, 0.5, 0.5, 1.0, 1.0,
                0.5, 0.5, -0.5, 1.0, 0.0,
                0.5, -0.5, -0.5, 0.0, 0.0,
                0.5, -0.5, -0.5, 0.0, 0.0,
                0.5, -0.5, 0.5, 0.0, 1.0,
                0.5, 0.5, 0.5, 1.0, 1.0,

                // Bottom Face
                -0.5, -0.5, -0.5, 0.0, 0.0,
                0.5, -0.5, -0.5, 1.0, 0.0,
                0.5, -0.5, 0.5, 1.0, 1.0,
                0.5, -0.5, 0.5, 1.0, 1.0,
                -0.5, -0.5, 0.5, 0.0, 1.0,
                -0.5, -0.5, -0.5, 0.0, 0.0,

                // Top Face
                -0.5, 0.5, -0.5, 0.0, 1.0,
                0.5, 0.5, -0.5, 1.0, 1.0,
                0.5, 0.5, 0.5, 1.0, 0.0,
                0.5, 0.5, 0.5, 1.0, 0.0,
                -0.5, 0.5, 0.5, 0.0, 0.0,
                -0.5, 0.5, -0.5, 0.0, 1.0,
            ]
        }
    }
}
