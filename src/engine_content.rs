use std::vec::Vec;
use rendering::ShaderProgram;
use ::system_manager::{System};
use ::*;
use cgmath::prelude::*;
use window::Window;
use ::cgmath::{Matrix4, Matrix3, Vector3, Vector2, Quaternion, Basis3, Euler};
use ::glfw::Key;
use input_system::InputSystem;
use rustberry_ecs::EcsRetrievable;
use engine_resources::MeshResource;

//============================
//Components:
//============================
#[derive(Clone, PartialEq, Debug)]
pub struct TransformCmp{
    pub position: Vector3<f32>,
    pub orientation: Quaternion<f32>,
    pub scale: Vector3<f32>,
}

#[derive(Clone, PartialEq, Debug, EcsRetrievable)]
pub struct CameraCmp{
    pub fovy: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,

    //TODO: Cache perspective/view matrices
}

impl TransformCmp{
    pub fn calculate_model_matrix(&self) -> Matrix4<f32>{
        //TODO: Why are all these conversions neccesairy?
        let rot_mat3:   Matrix3<f32> = Matrix3::from( Basis3::from_quaternion(&self.orientation) );
        let rot_mat:    Matrix4<f32> = Matrix4::from( rot_mat3 );
        let pos_mat:    Matrix4<f32> = Matrix4::from_translation(self.position);
        let scale_mat:  Matrix4<f32> = Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);

        pos_mat * rot_mat * scale_mat
    }
}

#[derive(Clone, PartialEq)]
pub struct MeshCmp{
    pub vertices: Vec<Vector3<f32>>,
    pub vbo: u32,
    pub shader: ShaderProgram,
}

impl MeshCmp{
    pub fn new(mesh_resource: &MeshResource, shader_program: ShaderProgram) -> MeshCmp{
        MeshCmp{
            vertices: mesh_resource.calculate_vertices(),
            vbo: 0,
            shader: shader_program,
        }
    }
}

///Holds data for screen/window information in this singleton
//#[derive(EcsRetrievable)]
pub struct ScreenDataCmp{
    pub mywindow: Window,
}

impl ScreenDataCmp {
    pub fn retrieve(ecs: &Ecs) -> Option<&ScreenDataCmp>{
        let mut screen_data_ids: Vec<EntityId> = Vec::new();
        let screen_data_filter = component_filter!(ScreenDataCmp);
        ecs.collect_with(&screen_data_filter, &mut screen_data_ids);
        if screen_data_ids.is_empty() {
            return None{};
        }
        let screen_data: & ScreenDataCmp = ecs.borrow(screen_data_ids[0]).expect("We querried with ecs.collect_with but it is not there?");
        Some(screen_data)
    }

    pub fn retrieve_mut(ecs: &mut Ecs) -> Option<&mut ScreenDataCmp>{
        let mut screen_data_ids: Vec<EntityId> = Vec::new();
        let screen_data_filter = component_filter!(ScreenDataCmp);
        ecs.collect_with(&screen_data_filter, &mut screen_data_ids);
        if screen_data_ids.is_empty() {
            return None{};
        }
        let screen_data: &mut ScreenDataCmp = ecs.borrow_mut(screen_data_ids[0]).expect("We querried with ecs.collect_with but it is not there?");
        Some(screen_data)
    }
}

#[derive(Clone, PartialEq, Debug, EcsRetrievable)]
pub struct TimeCmp{
    pub current_time: f64,
    pub delta_time: f64,
}

//============================
//Systems:
//============================
pub struct FreelookCameraSystem{
    pub movement_speed: f32,
    pub rotation_speed: f32,

    pub active: bool,
}

impl FreelookCameraSystem { 

}

impl System for FreelookCameraSystem{
    fn init(&mut self, ecs: &mut Ecs){
        println!("FreelookCameraSystem is initialized!");
    }
    fn update(&mut self, ecs: &mut Ecs, delta_time: f64){
        if !self.active{ return (); }

        //Get the input ready:

        let mut camera_ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(TransformCmp, CameraCmp);
        ecs.collect_with(&filter, &mut camera_ids);
        for camera_id in camera_ids{
            let mut camera_transform: TransformCmp = ecs.get(camera_id).unwrap();
            //let mut camera_cmp: CameraCmp = ecs.get(camera_id).unwrap();


            //Get the correct absolutemovement
            let mut mouse_movement: Vector2<f32> = Vector2::zero();
            let mut movement: Vector3<f32> = Vector3::zero();
            {
                let input_sys = InputSystem::retrieve(ecs).expect("Cannot retrieve the input system from the ecs, try adding the screen_data to the ecs");

                if input_sys.key_down(Key::Left) {
                    mouse_movement += Vector2::new(-1.0, 0.0);
                } if input_sys.key_down(Key::Right) {
                    mouse_movement += Vector2::new(1.0, 0.0);
                } if input_sys.key_down(Key::Up) {
                    mouse_movement += Vector2::new(0.0, -1.0);
                } if input_sys.key_down(Key::Down) {
                    mouse_movement += Vector2::new(0.0, 1.0);
                }

                if input_sys.key_down(Key::A) {
                    movement += Vector3::new(-1.0, 0.0, 0.0);
                } if input_sys.key_down(Key::D) {
                    movement += Vector3::new(1.0, 0.0, 0.0);
                } if input_sys.key_down(Key::W) {
                    movement += Vector3::new(0.0, 0.0, -1.0);
                } if input_sys.key_down(Key::S) {
                    movement += Vector3::new(0.0, 0.0, 1.0);
                } if input_sys.key_down(Key::Q) {
                    movement += Vector3::new(0.0, 1.0, 0.0);                    
                } if input_sys.key_down(Key::E) {
                    movement += Vector3::new(0.0, -1.0, 0.0);                    
                }
                movement *= self.movement_speed * delta_time as f32;
                mouse_movement *= self.rotation_speed * delta_time as f32;
            }

            //Rotate the absolute movement vectors to the rotation of the camera:
            let rot3 = Basis3::from_quaternion(&camera_transform.orientation);
            movement = rot3.rotate_vector(movement);
            
            //Rotate the camera with mouse (or arrow keys)
            camera_transform.orientation = camera_transform.orientation * Quaternion::from(Euler {
                x: Deg(-mouse_movement.y),
                y: Deg(-mouse_movement.x),
                z: Deg(0.0)
            });

            //set the transform again:
            camera_transform.position += movement;
            let _ = ecs.set(camera_id, camera_transform).unwrap();
        }
    }
}

