use cgmath::Vector3;
use crate::cube::Cube;
use crate::game_specs::{CHUNK_SIZE, MAX_CHUNK_HEIGHT};

pub struct ChunkColumn {
    x_location : f32,
    z_location : f32,
    pub cubes : Vec<Cube>
}

impl ChunkColumn {
    fn new(x_location : f32, z_location : f32) -> Self {

        let mut cubes : Vec<Cube> = Vec::new();

        for i in 0..MAX_CHUNK_HEIGHT {
            cubes.push(
                Cube::new(Vector3::new(x_location, i as f32, z_location), None)
            );
        }

        ChunkColumn {
            x_location,
            z_location,
            cubes
        }
    }
}

//one chunk is a 16x16 of columns
pub struct Chunk {
    x_origin : f32,
    z_origin : f32,
    pub columns : Vec<ChunkColumn>,
    //columns : [[ChunkColumn; CHUNK_SIZE] ; CHUNK_SIZE]
}

impl Chunk {
    pub fn new(x_origin : f32, z_origin : f32) -> Self {

        let mut columns = Vec::new();

        let x = x_origin as usize;
        let z = z_origin as usize;

        for i in x..CHUNK_SIZE {
            for j in z..CHUNK_SIZE {
                columns.push(ChunkColumn::new(i as f32, j as f32));
            }
        }

        Chunk {
            x_origin,
            z_origin,
            columns
        }
    }
}
