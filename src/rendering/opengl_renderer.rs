use engine_content::{TransformCmp, CameraCmp, MeshCmp};
use recs::*;
//Otherwise the component_filter! macro doesnt work:
use ::*;
use cgmath;
use cgmath::prelude::*;
use cgmath::{Matrix4, Rad, Deg};
use ::gl;
use ::gl::types::*;


use std::ffi::CString;
use std::ptr;

pub trait Renderer{
    fn render(&mut self, camera_transform: &TransformCmp, camera_cmp: &CameraCmp, ecs: &mut Ecs);
}

pub struct OpenglRenderer{ }
impl OpenglRenderer{
    pub fn new() -> OpenglRenderer{
        OpenglRenderer{}
    }

    pub fn render_mesh(&mut self, mvp: &Matrix4<f32>, mesh_cmp: &MeshCmp){
        unsafe{
            gl::UseProgram(mesh_cmp.shader.handle());
            gl::BindBuffer(gl::ARRAY_BUFFER, mesh_cmp.vbo);

            let pos_attr = gl::GetAttribLocation(mesh_cmp.shader.handle(), CString::new("position").unwrap().as_ptr());
            gl::EnableVertexAttribArray(pos_attr as GLuint);
            gl::VertexAttribPointer(
                pos_attr as GLuint,
                3,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                0,
                ptr::null(),
            );

            let mvp_location = gl::GetUniformLocation(mesh_cmp.shader.handle(), CString::new("u_MVP").unwrap().as_ptr());
            gl::UniformMatrix4fv(mvp_location, 1, gl::FALSE, &mvp[0][0]);

            gl::DrawArrays(gl::TRIANGLES, 0, mesh_cmp.vertices.len() as i32);
        }
    }
}
impl Renderer for OpenglRenderer{
    fn render(&mut self, camera_transform: &TransformCmp, camera_cmp: &CameraCmp, ecs: &mut Ecs){
        let perspective_mat = cgmath::perspective(Rad::from(Deg(camera_cmp.fovy)), camera_cmp.aspect, camera_cmp.near, camera_cmp.far);
        let view_mat = camera_transform.calculate_model_matrix().invert().expect("Cannot invert view matrix!");
        let vp = perspective_mat * view_mat;

        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(TransformCmp, MeshCmp);
        ecs.collect_with(&filter, &mut ids);
        for id in ids{
            let mesh_transform: TransformCmp = ecs.get(id).unwrap();
            let mesh_cmp: MeshCmp = ecs.get(id).unwrap();
            let mvp = vp * mesh_transform.calculate_model_matrix();
            self.render_mesh(&mvp, &mesh_cmp);
        }
    }
}
