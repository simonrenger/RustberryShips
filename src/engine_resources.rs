use resource_manager::Resource;
use std::vec::Vec;
use ::cgmath::{Vector3, Quaternion, Matrix4, Matrix3, Basis3, Vector4};
use cgmath::prelude::*;


//TODO: Rename to something like: NonIndexedMeshResource
#[derive(Clone)]
pub struct MeshResource{
    pub raw_vertices: Vec<Vector3<f32>>,
    pub _materials: Vec<u32>, 

    pub orientation: Quaternion<f32>,
    pub scale: f32,
}

impl MeshResource{
    pub fn new() -> MeshResource{
        MeshResource{
            raw_vertices: Vec::new(),
            _materials: Vec::new(),

            orientation: Quaternion::one(),
            scale: 1.0,
        }
    }

    pub fn calculate_vertices(&self) -> Vec<Vector3<f32>>{
        let mut verts = self.raw_vertices.clone();

        let rot_mat3:   Matrix3<f32> = Matrix3::from( Basis3::from_quaternion(&self.orientation) );
        let rot_mat:    Matrix4<f32> = Matrix4::from( rot_mat3 );
        let scale_mat:  Matrix4<f32> = Matrix4::from_scale(self.scale);
        let transform_mat = rot_mat * scale_mat;

        for i in 0..verts.len() {
            let fuckyou = transform_mat * Vector4::new(verts[i].x, verts[i].y, verts[i].z, 0.0);
            verts[i] = Vector3::new(fuckyou.x, fuckyou.y, fuckyou.z);
        }

        verts
    }
}

