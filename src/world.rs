use cgmath::Vector3;
use crate::chunk::Chunk;
use crate::cube::Cube;

pub struct World {
    pub chunks : Vec<Chunk>,
    pub cube_positions : Vec<Vector3<f32>>
}

impl World {
    pub fn new() -> Self {
        //let cube = Cube::new(Vector3::new(0.0, 0.0, 0.0), );
        let mut chunks = Vec::new();
        // TODO just trying to get one to work
        chunks.push(Chunk::new(0.0, 0.0));
        chunks.push(Chunk::new(0.0, 0.0));

        let mut cube_positions = Vec::new();

        for chunk in &chunks {
            for column in &chunk.columns {
                for cube in &column.cubes {
                    cube_positions.push(cube.position);
                }
            }
        }

        World {
            chunks,
            cube_positions
        }
    }
}