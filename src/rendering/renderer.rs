use std::ffi::CString;
use std::mem;
use cgmath::{Matrix4};
use gl::types::{GLenum, GLfloat, GLsizei, GLuint};
use crate::core::lib::{polygon_mode};
use crate::creation::cube::{Cube, CubeType};
use crate::game_specs::{POLYGON_MODE};
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

            // the whole world is cubes so the vertex and index buffers will be the same for everything
            let vbo = define_buffer(gl::ARRAY_BUFFER, &Cube::VERTICES, gl::STATIC_DRAW);
            //let ibo = define_buffer(gl::ELEMENT_ARRAY_BUFFER, &Cube::INDICES, gl::STATIC_DRAW); // TODO index buffer doesn't work, I have the order wrong

            // define attribute pointers
            let stride = (5 * mem::size_of::<GLfloat>()) as GLsizei;
            self.define_attrib_pointers(stride);

            //assign shader sampler to texture unit
            self.shader_program.set_int(&CString::new("blockAtlas").unwrap(), 0);

            // only ever using one texture
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.block_atlas.id);
        }

        // "settings"
        unsafe { gl::ClearColor(0.60, 0.7, 0.9, 1.0); }
        polygon_mode(POLYGON_MODE);
    }

    // called from game window loop
    pub fn render(&mut self, projection : Matrix4<f32>, view : Matrix4<f32>, models : (Vec<Matrix4<f32>>, Vec<&CubeType>)) {
        // render
        unsafe {
            // clear buffers
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // pass to the shaders
            self.shader_program.set_mat4(&CString::new("projection").unwrap(), &projection);
            self.shader_program.set_mat4(&CString::new("view").unwrap(), &view);


            for i in 0..models.0.len() {
                // only send model to renderer if it isn't air
                if *models.1[i] != CubeType::AIR {
                    self.shader_program.set_mat4(&CString::new("model").unwrap(), &models.0[i]);
                }

                // send block type information to fragment shader
                match models.1[i]  {
                    CubeType::GRASS => {
                        self.shader_program.set_int(&CString::new("blockType").unwrap(), 1);
                    }
                    CubeType::STONE => {
                        self.shader_program.set_int(&CString::new("blockType").unwrap(), 2);
                    }
                    CubeType::DIRT => {
                        self.shader_program.set_int(&CString::new("blockType").unwrap(), 3);
                    }
                     // we don't want anything to render for air
                    _ => {}
                }

                // draw objects
                gl::DrawArrays(
                    gl::TRIANGLES,
                    0,
                    36
                );

                // TODO reorder the index array so this works
                // gl::DrawElements(
                //     gl::TRIANGLES,
                //     36,
                //     gl::UNSIGNED_INT,
                //     std::ptr::null(), // Indices are provided by the bound element array buffer
                // );
            }
        }
    }

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

// Generate and bind buffer objects for both VBO and IBO
fn define_buffer<T>(target: GLenum, array: &[T], draw_type: GLenum) -> GLuint {
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