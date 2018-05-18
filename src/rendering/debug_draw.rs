use ::gl;
use ::recs::{Ecs, EntityId};
//use ::gl::types::{GLint, GLuint};
use ::cgmath::{Vector3, Vector4, Matrix4};
use ::rendering::ShaderProgram;
use std::ffi::CString;
use std::mem;
use std::ptr;
use std::os::raw::c_void;
use std::any::Any;
use ::*;
use ::rustberry_ecs::EcsRetrievable;

use std::vec;

#[derive(Debug)]
struct PointLocation{
    pos: Vector3<f32>,
    color: Vector4<f32>,
}

#[derive(EcsRetrievable)]
pub struct DebugDraw{
    points: Vec<PointLocation>,
    lines: Vec<(PointLocation, PointLocation)>,

    shader: ShaderProgram,
    vbo: u32,
    //attribs
    position_location: gl::types::GLuint,
    color_location: gl::types::GLuint,
    //uniforms
    uniform: gl::types::GLint,
}

impl DebugDraw{
    pub fn new() -> DebugDraw{
        let mut vbo = 0;        
        unsafe{
            let shader = ShaderProgram::from_files("resources/shaders/debug_draw.vertexshader", "resources/shaders/debug_draw.fragshader");
            let pos_loc = gl::GetAttribLocation(shader.handle(), CString::new("a_position").unwrap().as_ptr()) as u32;
            let color_loc = gl::GetAttribLocation(shader.handle(), CString::new("a_color").unwrap().as_ptr()) as u32;
            let vp_loc = gl::GetUniformLocation(shader.handle(), CString::new("u_VP").unwrap().as_ptr());
            gl::GenBuffers(1, &mut vbo);

            DebugDraw{
                points: Vec::<PointLocation>::new(),
                lines: Vec::<(PointLocation, PointLocation)>::new(),
                shader: shader,
                vbo: vbo,
                position_location: pos_loc,
                color_location: color_loc,
                uniform: vp_loc,
            }
        }
    }

    pub fn add_point(&mut self, pos: Vector3<f32>, color: Vector4<f32>){
        self.points.push(PointLocation{pos, color});
    }

    pub fn add_red_point(&mut self, pos: Vector3<f32>){
        let red = Vector4{x: 1.0, y: 0.0, z: 0.0, w: 0.0};
        self.points.push(PointLocation{pos: pos, color: red});
    }

    pub fn add_line(&mut self, from: Vector3<f32>, to: Vector3<f32>, color: Vector4<f32>){
        self.lines.push((PointLocation{pos: from, color: color}, PointLocation{pos: to, color: color}));
    }
    
    pub fn add_red_line(&mut self, from: Vector3<f32>, to: Vector3<f32>){
        let red = Vector4{x: 1.0, y: 0.0, z: 0.0, w: 0.0};
        let segment = (PointLocation{pos: from, color: red}, PointLocation{pos: to, color: red});
        self.lines.push(segment);
    }

    pub fn render(&self, vp_mat: &Matrix4<f32>){
        //TODO: Render points

        unsafe{
            gl::UseProgram(self.shader.handle());
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(gl::ARRAY_BUFFER, (mem::size_of::<PointLocation>() * self.lines.len() * 2) as gl::types::GLsizeiptr, mem::transmute(&self.lines[0].0), gl::DYNAMIC_DRAW);
            
            gl::EnableVertexAttribArray(self.position_location);
            gl::EnableVertexAttribArray(self.color_location);
            gl::VertexAttribPointer(self.position_location, 3, gl::FLOAT, gl::FALSE, mem::size_of::<gl::types::GLfloat>() as i32 * 7, ptr::null());
            gl::VertexAttribPointer(self.color_location, 4, gl::FLOAT, gl::FALSE, mem::size_of::<gl::types::GLfloat>() as i32 * 7,  (mem::size_of::<gl::types::GLfloat>() * 3) as *const c_void);

            gl::UniformMatrix4fv(self.uniform, 1, gl::FALSE, &vp_mat[0][0]);

            gl::DrawArrays(gl::LINES, 0, (self.lines.len() * 2) as i32);
        }
    }

    pub fn clear(&mut self){
        self.points.clear();
        self.lines.clear();
    }
}

impl Drop for DebugDraw{
    fn drop(&mut self){
        unsafe{
            gl::DeleteBuffers(1, &mut self.vbo);
        }
    }
}

