use std::collections::HashMap;
use cgmath::{Vector2, Vector3};
use rand::random;
use crate::creation::cube::{Cube, CubeType};
use crate::creation::cube::CubeType::{AIR, GRASS, STONE};
use crate::game_specs::{CHUNK_SIZE, MAX_CHUNK_HEIGHT};
use crate::rendering::mesh::Mesh;
use crate::rendering::texture::Texture;

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
                    ), CubeType::random())); //todo figure out how you're doing block types
                }
            }
        }

        // // TODO testing...set random cubes
        // for mut cube in cubes {
        //     cube._type = CubeType::random();
        // }

        Chunk {
            position,
            cubes
        }
    }

    // pub fn generate_mesh(&self) -> Mesh {
    //     // let mut vertices = Vec::new();
    //     // let mut indices = Vec::new();
    //     // let mut vertex_index = 0;
    //     // let cube_meshes: HashMap<CubeType, Mesh> = create_cube_meshes();
    //     //
    //     // for cube in &self.cubes {
    //     //     if cube._type == AIR {
    //     //         continue; // Skip empty cubes
    //     //     }
    //     //
    //     //     if let Some(cube_mesh) = cube_meshes.get(&cube._type) {
    //     //         // Translate and add the vertices and indices of the cube mesh to the chunk mesh
    //     //         for mut vertex in cube_mesh.vertices {
    //     //             // You might need to transform the vertices based on cube's position
    //     //             vertex.position += cube.position;
    //     //             vertices.push(*vertex); // maybe *vertex?
    //     //         }
    //     //         for index in &cube_mesh.indices {
    //     //             indices.push(vertex_index + *index);
    //     //         }
    //     //         vertex_index += cube_mesh.vertices.len() as u32;
    //     //     }
    //     // }
    //     //
    //     // // Create the Mesh structure for the chunk
    //     // let mut mesh = Mesh {
    //     //     vertices,
    //     //     indices,
    //     //     texture: self.texture.clone(), // Clone or reference the chunk's texture
    //     //     VAO: 0, // Initialize with appropriate values
    //     //     VBO: 0,
    //     //     EBO: 0,
    //     // };
    //     //
    //     // // TODO: Create and bind VAO, VBO, and EBO, and send data to GPU
    //     //
    //     // mesh
    // }

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

// create cube meshes for each block type
// fn create_cube_meshes() -> HashMap<CubeType, Mesh> {
//     // // Implement the logic to create cube meshes for different block types
//     // let mut cube_meshes = HashMap::new();
//     //
//     // // For each block type, generate its mesh and store it in the HashMap
//     // let dirt_mesh = Mesh {
//     //     vertices: vec![/* Define vertices for dirt block */],
//     //     indices: vec![/* Define indices for dirt block */],
//     //
//     //     texture: Texture {},
//     //     VAO: 0,
//     //     VBO: 0,
//     //     EBO: 0,
//     // };
//     // cube_meshes.insert(CubeType::DIRT, dirt_mesh);
//     //
//     // // Repeat this for all block types
//     //
//     // cube_meshes
// }
