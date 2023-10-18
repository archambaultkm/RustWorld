use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::Read;
use cgmath::{Matrix, Matrix4};
use gl::types::{GLchar, GLenum, GLint, GLuint};

pub struct Shader {
    //program id
    pub id: u32
}

impl Shader {
    pub fn new(vertex_file_path : &str, fragment_file_path : &str) -> Shader {

        let mut shader_program = Shader { id: 0};

        let vertex_shader = shader_code_from_file(vertex_file_path);
        let fragment_shader = shader_code_from_file(fragment_file_path);

        // Compile and link shaders
        let vertex_shader = compile_shader(vertex_shader, gl::VERTEX_SHADER);
        let fragment_shader = compile_shader(fragment_shader, gl::FRAGMENT_SHADER);
        shader_program.id = create_shader_program(vertex_shader, fragment_shader);

        shader_program
    }

    pub unsafe fn set_bool(&self, name: &CStr, value: bool) {
        gl::Uniform1i(
            gl::GetUniformLocation(self.id, name.as_ptr()),
            value as i32
        );
    }
    pub unsafe fn set_int(&self, name: &CStr, value: i32) {
        gl::Uniform1i(gl::GetUniformLocation(
            self.id,
            name.as_ptr()
        ), value);
    }
    pub unsafe fn set_float(&self, name: &CStr, value: f32) {
        gl::Uniform1f(
            gl::GetUniformLocation(self.id, name.as_ptr()),
            value
        );
    }

    pub unsafe fn set_mat4(&self, name: &CStr, matrix : &Matrix4<f32>) {
        let location = gl::GetUniformLocation(
            self.id,
            name.as_ptr()
        );

        gl::UniformMatrix4fv(
            location,
            1,
            gl::FALSE,
            matrix.as_ptr()
        );
    }
}

fn shader_code_from_file(file_path : &str) -> CString {

    let mut file = File::open(file_path)
        .unwrap_or_else(|_| panic!("Failed to open {}", file_path));

    let mut code = String::new();

    file
        .read_to_string(&mut code)
        .expect("failed to read shader");

    let code = CString::new(code.as_bytes()).unwrap();

    code
}

fn compile_shader(source: CString, shader_type: GLenum) -> GLuint {
    unsafe {
        // Create a new shader object
        let shader = gl::CreateShader(shader_type);

        // Set the shader source and compile it
        gl::ShaderSource(shader, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);

        // Check for compilation errors
        let mut success = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

        if success != gl::TRUE as GLint {
            // Compilation failed, get the error message
            let mut log_length = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut log_length);

            let mut log = Vec::with_capacity(log_length as usize);
            log.set_len(log_length as usize - 1); // Subtract 1 to ignore the null terminator

            gl::GetShaderInfoLog(shader, log_length, std::ptr::null_mut(), log.as_mut_ptr() as *mut GLchar);

            let error_message = String::from_utf8_lossy(&log);
            println!("Shader compilation error: {}", error_message);
        }

        shader
    }
}

fn create_shader_program(vertex_shader: GLuint, fragment_shader: GLuint) -> GLuint {
    unsafe {
        // Create a new shader program
        let shader_program = gl::CreateProgram();

        // Attach the vertex and fragment shaders to the program
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);

        // Link the shader program
        gl::LinkProgram(shader_program);

        // Check for linking errors
        let mut success = gl::FALSE as GLint;
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);

        if success != gl::TRUE as GLint {
            // Linking failed, get the error message
            let mut log_length = 0;
            gl::GetProgramiv(shader_program, gl::INFO_LOG_LENGTH, &mut log_length);

            let mut log = Vec::with_capacity(log_length as usize);
            log.set_len(log_length as usize - 1); // Subtract 1 to ignore the null terminator

            gl::GetProgramInfoLog(shader_program, log_length, std::ptr::null_mut(), log.as_mut_ptr() as *mut GLchar);

            let error_message = String::from_utf8_lossy(&log);
            println!("Shader program linking error: {}", error_message);
        }

        // Detach and delete the individual shaders since they are now part of the program
        gl::DetachShader(shader_program, vertex_shader);
        gl::DetachShader(shader_program, fragment_shader);
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        shader_program
    }
}