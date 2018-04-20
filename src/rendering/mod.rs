pub mod debug_draw; 

use super::gl;
use super::gl::types::*;
use std::str;
use std::ffi::CString;
use std::ptr;

use std::env;
use std::fs::File;
use std::io::prelude::*;

// use super::*;
// use std::sync::mpsc::Receiver

// pub struct Renderer{
// 	pub window: glfw::Window,

// 	//TODO: Make this a seperate struct
// 	pub events: Receiver<(f64, WindowEvent)>
// }

// impl Renderer {
// 	pub fn new(_glfw: &mut glfw::Glfw) -> Renderer{
// 		let (mut window, events) = _glfw.create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
//         .expect("Failed to create GLFW window.");

// 		Renderer{
// 			window: window,
// 			events: events,
// 		}
// 	}

// 	fn initialize(&mut self){

// 	}

// 	fn draw_world(&mut self){

// 	}
// }

// impl Drop for Renderer {
// 	fn drop(&mut self) {

// 	}
// }



pub struct ShaderProgram{
    id: GLuint,
}

impl ShaderProgram {
    pub fn from_files(vertex_shader: &str, fragment_shader: &str) -> ShaderProgram{
        let mut vertex_file = match File::open(vertex_shader){
            Ok(file) => file,
            Err(_) => panic!(String::from("Cannot open file: ") + vertex_shader),
        };

        let mut vertex_shader_src = String::new();
        vertex_file.read_to_string(&mut vertex_shader_src).expect("Something went wrong reading the file");

        let mut fragment_file = match File::open(fragment_shader){
            Ok(file) => file,
            Err(_) => panic!(String::from("Cannot open file: ") + fragment_shader),
        };
        let mut fragment_shader_src = String::new();
        fragment_file.read_to_string(&mut fragment_shader_src).expect("Something went wrong reading the file");


        ShaderProgram::from_source(&vertex_shader_src, &fragment_shader_src)
    }

    pub fn from_source(vertex_shader_src: &str, fragment_shader_src: &str) -> ShaderProgram{
        let vs = ShaderProgram::compile_shader(vertex_shader_src, gl::VERTEX_SHADER);
        let fs = ShaderProgram::compile_shader(fragment_shader_src, gl::FRAGMENT_SHADER);
        let program = ShaderProgram::link_program(vs, fs);

        ShaderProgram{
            id: program,
        }
    }

    pub fn handle(&self) -> GLuint{
        self.id
    }

    fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    let shader;
    unsafe {
        shader = gl::CreateShader(ty);
        // Attempt to compile the shader
        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        // Get the compile status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(
                shader,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "{}",
                str::from_utf8(&buf)
                    .ok()
                    .expect("ShaderInfoLog not valid utf8")
            );
        }
    }
    shader
}

fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);
        // Get the link status
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(
                program,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "{}",
                str::from_utf8(&buf)
                    .ok()
                    .expect("ProgramInfoLog not valid utf8")
            );
        }
        program
    }
}
}
