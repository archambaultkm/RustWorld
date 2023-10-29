use std::ffi::{c_void, CString};
use std::{mem, ptr};
use std::mem::{size_of};
use cgmath::{Vector2, Vector3, Zero};
use gl::types::{GLfloat, GLsizei, GLuint};
use crate::rendering::shader::Shader;
use crate::rendering::texture::Texture;

pub struct Vertex {
    pub position : Vector3<f32>,
    pub normal : Vector3<f32>,
    pub texture_coords : Vector2<f32>
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            position : Vector3::zero(),
            normal : Vector3::zero(),
            texture_coords : Vector2::zero()
        }
    }
}

pub struct Mesh {
    pub vertices : Vec<Vertex>,
    pub indices : Vec<u32>,
    pub texture : Texture,
    pub VAO : u32,

    // for rendering
    VBO : u32,
    EBO : u32
}

impl Mesh {
    pub fn new(vertices : Vec<Vertex>, indices : Vec<u32>, texture : Texture, shader : Shader) -> Mesh {
        let mut mesh = Mesh {
            vertices, indices, texture,
            VAO: 0, VBO: 0, EBO: 0
        };

        // set mesh buffers and attrib pointers
        unsafe { mesh.setup_mesh(shader) }

        mesh
    }

    unsafe fn setup_mesh(&mut self, shader : Shader) {
        // create buffers/arrays
        gl::GenVertexArrays(1, &mut self.VAO);
        gl::GenBuffers(1, &mut self.VBO);
        gl::GenBuffers(1, &mut self.EBO);

        gl::BindVertexArray(self.VAO);
        // load data into vertex buffers
        gl::BindBuffer(gl::ARRAY_BUFFER, self.VBO);

        let size = (self.vertices.len() * size_of::<Vertex>()) as isize;
        let data = &self.vertices[0] as *const Vertex as *const c_void;
        gl::BufferData(gl::ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.EBO);
        let size = (self.indices.len() * size_of::<u32>()) as isize;
        let data = &self.indices[0] as *const u32 as *const c_void;
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

        let pos_attr_location = gl::GetAttribLocation(
            shader.id,
            CString::new("position").unwrap().as_ptr()
        );

        let texture_attr_location = gl::GetAttribLocation(
            shader.id,
            CString::new("texture").unwrap().as_ptr()
        );

        let stride = (5 * mem::size_of::<GLfloat>()) as GLsizei;

        // position attribute
        gl::VertexAttribPointer(
            pos_attr_location as GLuint,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(pos_attr_location as GLuint);

        // texture attribute
        gl::VertexAttribPointer(
            texture_attr_location as GLuint,
            2,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (3 * mem::size_of::<GLfloat>()) as *const std::ffi::c_void,
        );
        gl::EnableVertexAttribArray(texture_attr_location as GLuint);

        gl::BindVertexArray(0);
    }
}
