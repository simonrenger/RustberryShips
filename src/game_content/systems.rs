use system_manager::System;
use recs::Ecs;
use ::*;
use rendering::debug_draw::DebugDraw;
use super::components::TilemapCmp;
use rustberry_ecs::EcsRetrievable;
use ::cgmath::{Vector3, Deg, Euler, Quaternion};
use engine_content::{CameraCmp, TransformCmp, TimeCmp};
use input_system::InputSystem;

pub struct TilemapSystem{

}

impl System for TilemapSystem{
    fn init(&mut self, ecs: &mut Ecs){
        println!("TilemapSystem is initialized!");
    }
    fn update(&mut self, ecs: &mut Ecs, _delta_time: f64){
        //Get the tilemap
        let filter = component_filter!(TilemapCmp);
        let mut entity_ids: Vec<EntityId> = Vec::new();
        ecs.collect_with(&filter, &mut entity_ids);
        if entity_ids.is_empty() {
            println!("Cannot find a TilemapCmp in the scene! please add one!");
            return ();
        }
        let tilemap: TilemapCmp = ecs.get(entity_ids[0]).unwrap();
        

        let db_draw = DebugDraw::retrieve_mut(ecs).unwrap();
        //Visualise the tilemap
        const TILE_SCALE: f32 = 10.0;
        for x in 0..tilemap.width+1{
            db_draw.add_red_line(
                Vector3::new(x as f32 * TILE_SCALE, 0.0, 0.0), 
                Vector3::new(x as f32 * TILE_SCALE, 0.0, tilemap.height as f32 * TILE_SCALE)
                );
        }
        for y in 0..tilemap.height+1{
            db_draw.add_red_line(
                Vector3::new(0.0, 0.0, y as f32 * TILE_SCALE), 
                Vector3::new(tilemap.width as f32 * TILE_SCALE, 0.0, y as f32 * TILE_SCALE)
                );
        }
    }
}


pub struct TopDownCameraSystem{
    tilt: Deg<f32>,
    active: bool,

    _move_delay: f64,
    _last_move: f64,
}

impl TopDownCameraSystem{
    ///Tilt in deg
    pub fn new(tilt: f32) -> TopDownCameraSystem{
        TopDownCameraSystem{
            tilt: Deg(tilt),
            active: true,

            _move_delay: 0.2,
            _last_move: -999.0,
        }
    }
}

impl System for TopDownCameraSystem{
    fn init(&mut self, ecs: &mut Ecs){
        let camera_entity = CameraCmp::retrieve_entity(ecs).expect("Cannot find a CameraCmp in the scene, did you forget to add it?");
        let mut cam_tr: TransformCmp = ecs.get(camera_entity).expect("The Camera entity does not have a TransformCmp, Please add it");
        cam_tr.orientation = Quaternion::from(Euler{x: self.tilt, y: Deg(0.0), z: Deg(0.0)});
        let _ = ecs.set(camera_entity, cam_tr);
    }

    fn update(&mut self, ecs: &mut Ecs, _delta_time: f64){
        if !self.active { return (); }

        let current_time: f64 = TimeCmp::retrieve(ecs).unwrap().current_time;
        if self._last_move + self._move_delay < current_time {
            let camera_entity = CameraCmp::retrieve_entity(ecs).expect("Cannot find a CameraCmp in the scene, did you forget to add it?");
            let mut cam_tr: TransformCmp = ecs.get(camera_entity).expect("The Camera entity does not have a TransformCmp, Please add it");

            const TILE_SCALE: f32 = 10.0;

            let mut movement: Vector3<f32> = Vector3::zero();
            {
                let input_sys = InputSystem::retrieve(ecs).expect("Cannot retrieve the input system from the ecs, try adding the screen_data to the ecs");

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
                movement *= TILE_SCALE;
            }

            if movement.magnitude2().abs() > 0.01 {
                self._last_move = current_time;
            }
            
            //TODO: Is there a situation where I do want to transform the movement vector?
            //movement = cam_tr.orientation * movement;
            cam_tr.position += movement;
            let _ = ecs.set(camera_entity, cam_tr);
        }
    }
}