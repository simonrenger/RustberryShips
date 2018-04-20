use ::cgmath::{Matrix4, Vector3, Quaternion};
use ::cgmath::prelude::*;

pub struct TransformCmp{
    pub position: Vector3<f32>,
    pub orientation: Quaternion<f32>,
    pub scale: Vector3<f32>,
}

pub struct CameraCmp{
    pub fovy: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,

    //TODO: Cache perspective/view matrices
}

impl TransformCmp{
    pub fn calculate_model_matrix(&self) -> Matrix4<f32>{
        //TODO: Use rotation/scale too

        Matrix4::from_translation(self.position)
    }
}