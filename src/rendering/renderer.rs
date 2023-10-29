use std::ffi::CString;
use std::mem;
use cgmath::{Matrix4, Vector3, Vector4};
use gl::types::{GLenum, GLfloat, GLsizei, GLuint};
use crate::core::lib::{polygon_mode};
use crate::creation::block_config::{load_block_config};
use crate::creation::cube::{Cube, CubeType};
use crate::creation::cube::CubeType::AIR;
use crate::game_specs::{CHUNK_SIZE, POLYGON_MODE};
use crate::rendering::shader::Shader;
use crate::rendering::texture::Texture;
use crate::creation::world::World;

pub struct Renderer {
    shader_program : Shader,
    vao : GLuint,
    block_atlas: Texture
}

impl Renderer {

    pub fn new() -> Self {

        let shader_program = Shader::new("shaders/shader.vert", "shaders/shader.frag");
        let block_atlas = unsafe { Texture::new("resources/textures/spritesheet.png", true) };

        Renderer {
            shader_program,
            vao: 0,
            block_atlas,
        }
    }

    pub fn init_renderer(&mut self, world : World) {
        unsafe {
            gl::UseProgram(self.shader_program.id);
            gl::Enable(gl::DEPTH_TEST);
            //gl::Enable(gl::CULL_FACE);
            //gl::CullFace(gl::BACK);

            // Generate and bind vertex array object (VAO)
            gl::GenVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);

            for chunk in &world.chunks {
                for cube in &chunk.cubes {
                    // Generate and bind vertex buffer object (VBO)
                    define_buffer(
                        gl::ARRAY_BUFFER,
                        &cube.vertices,
                        gl::STATIC_DRAW
                    );
                }
            }

            // define attribute pointers
            //TODO hard-coding stride size for now
            let stride = (5 * mem::size_of::<GLfloat>()) as GLsizei;
            self.define_attrib_pointers(stride);

            //assign shader sampler to texture unit
            self.shader_program.set_int(&CString::new("blockAtlas").unwrap(), 0);

            // only ever using one texture
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.block_atlas.id);

            if let Ok(block_config) = load_block_config() {
                // TODO needs to apply to all texture types/ sides- handle in the fragment shader.
                let (atlas_x, atlas_y, atlas_w, atlas_h) = block_config.get_texture_coordinates("grass", "top");
                self.shader_program.set_int(&CString::new("blockType").unwrap(), 0);
                self.shader_program.set_float(&CString::new("atlasX").unwrap(), atlas_x);
                self.shader_program.set_float(&CString::new("atlasY").unwrap(), atlas_y);
                self.shader_program.set_float(&CString::new("atlasW").unwrap(), atlas_w);
                self.shader_program.set_float(&CString::new("atlasH").unwrap(), atlas_h);
            }
        }

        // "settings"
        unsafe { gl::ClearColor(0.7, 0.7, 0.8, 1.0); }
        polygon_mode(POLYGON_MODE);
    }

    // called from game window loop
    pub fn render(&mut self, projection : Matrix4<f32>, view : Matrix4<f32>, models : (Vec<Matrix4<f32>>, Vec<CubeType>) ) {
        // render
        unsafe {
            // clear buffers
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // pass to the shaders
            self.shader_program.set_mat4(&CString::new("projection").unwrap(), &projection);
            self.shader_program.set_mat4(&CString::new("view").unwrap(), &view);

            // draw
            gl::BindVertexArray(self.vao);

            for i in 0..models.0.len() {

                self.shader_program.set_mat4(&CString::new("model").unwrap(), &models.0[i]);
                self.shader_program.set_int(&CString::new("blockType").unwrap(), models.1[i] as i32);

                gl::DrawArrays(
                    gl::TRIANGLES,
                    0,
                    36
                );
            }
        }
    }

    //related to the vertex shader
    unsafe fn define_attrib_pointers(&self, stride : GLsizei) {
        let pos_attr_location = gl::GetAttribLocation(
            self.shader_program.id,
            CString::new("position").unwrap().as_ptr()
        );

        let texture_attr_location = gl::GetAttribLocation(
            self.shader_program.id,
            CString::new("texture").unwrap().as_ptr()
        );

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
    }
}

//generate and bind buffer objects for both VBO and EBO
fn define_buffer<T>(target: GLenum, array : &[T], draw_type : GLenum) -> GLuint {
    let mut buffer_object = 0;
    unsafe {
        gl::GenBuffers(1, &mut buffer_object);
        gl::BindBuffer(target, buffer_object);
        gl::BufferData(
            target,
            (array.len() * std::mem::size_of::<T>()) as isize,
            array.as_ptr() as *const std::ffi::c_void,
            draw_type,
        );
    }

    buffer_object
}