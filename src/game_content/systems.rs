use system_manager::System;
use recs::Ecs;
use ::*;
use rendering::debug_draw::DebugDraw;
use super::components::*;
use rustberry_ecs::EcsRetrievable;
use ::cgmath::{Vector3, Deg, Euler, Quaternion};
use engine_content::{CameraCmp, TransformCmp, TimeCmp};
use input_system::InputSystem;
use num;

pub struct TilemapSystem{

}

impl System for TilemapSystem{
    fn init(&mut self, ecs: &mut Ecs){
        println!("TilemapSystem is initialized!");
    }
    fn update(&mut self, ecs: &mut Ecs, _delta_time: f64){
        //Get the tilemap
        if let Some(tilemap_entity) = TilemapCmp::retrieve_entity(ecs){
            let tilemap: TilemapCmp = ecs.get(tilemap_entity).unwrap();
            let db_draw = DebugDraw::retrieve_mut(ecs).unwrap();
            //Visualise the tilemap
            let tile_scale: f32 = tilemap.tile_size;
            for x in 0..tilemap.width+1{
                let xpos = x as f32 * tile_scale;
                db_draw.add_red_line(
                    Vector3::new(xpos, 0.0, 0.0), 
                    Vector3::new(xpos, 0.0, tilemap.height as f32 * tile_scale)
                    );
            }
            for y in 0..tilemap.height+1{
                let ypos = y as f32 * tile_scale;
                db_draw.add_red_line(
                    Vector3::new(0.0, 0.0, ypos), 
                    Vector3::new(tilemap.width as f32 * tile_scale, 0.0, ypos)
                    );
            }
        }else{
            println!("Cannot find TilemapCmp, please add one");
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

            let tile_scale: f32 = TilemapCmp::retrieve(ecs).unwrap().tile_size;            

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
                movement *= tile_scale;
            }

            if movement.magnitude2() > 0.01 {
                self._last_move = current_time;
            }
            
            //TODO: Is there a situation where I do want to transform the movement vector?
            //movement = cam_tr.orientation * movement;
            cam_tr.position += movement;
            let _ = ecs.set(camera_entity, cam_tr);
        }
    }
}

#[derive(PartialEq)]
enum PlacingState{
    MyTeam, OtherTeam, Done
}

static TANK_VERTICES: [Vector3<f32>; 36] = [
    Vector3{x: -1.0, y: -1.0, z: -1.0},
    Vector3{x: -1.0, y: -1.0, z:  1.0},
    Vector3{x: -1.0, y:  1.0, z:  1.0}, 
    Vector3{x: 1.0,  y: 1.0,  z: -1.0}, 
    Vector3{x: -1.0, y: -1.0, z: -1.0},
    Vector3{x: -1.0, y:  1.0, z: -1.0}, 
    Vector3{x: 1.0,  y: -1.0, z:  1.0},
    Vector3{x: -1.0, y: -1.0, z: -1.0},
    Vector3{x: 1.0,  y: -1.0, z: -1.0},
    Vector3{x: 1.0,  y: 1.0,  z: -1.0},
    Vector3{x: 1.0,  y: -1.0, z: -1.0},
    Vector3{x: -1.0, y: -1.0, z: -1.0},
    Vector3{x: -1.0, y: -1.0, z: -1.0},
    Vector3{x: -1.0, y:  1.0, z:  1.0},
    Vector3{x: -1.0, y:  1.0, z: -1.0},
    Vector3{x: 1.0,  y: -1.0, z:  1.0},
    Vector3{x: -1.0, y: -1.0, z:  1.0},
    Vector3{x: -1.0, y: -1.0, z: -1.0},
    Vector3{x: -1.0, y:  1.0, z:  1.0},
    Vector3{x: -1.0, y: -1.0, z:  1.0},
    Vector3{x: 1.0,  y: -1.0, z:  1.0},
    Vector3{x: 1.0,  y: 1.0,  z: 1.0 },
    Vector3{x: 1.0,  y: -1.0, z: -1.0},
    Vector3{x: 1.0,  y: 1.0,  z: -1.0},
    Vector3{x: 1.0,  y: -1.0, z: -1.0},
    Vector3{x: 1.0,  y: 1.0,  z: 1.0 },
    Vector3{x: 1.0,  y: -1.0, z:  1.0},
    Vector3{x: 1.0,  y: 1.0,  z: 1.0 },
    Vector3{x: 1.0,  y: 1.0,  z: -1.0},
    Vector3{x: -1.0, y:  1.0, z: -1.0},
    Vector3{x: 1.0,  y: 1.0,  z: 1.0 },
    Vector3{x: -1.0, y:  1.0, z: -1.0},
    Vector3{x: -1.0, y:  1.0, z:  1.0},
    Vector3{x: 1.0,  y: 1.0,  z: 1.0 },
    Vector3{x: -1.0, y:  1.0, z:  1.0},
    Vector3{x: 1.0,  y: -1.0, z:  1.0}];

pub struct BoardManagerSystem{
    pub cursor_x: u32,
    pub cursor_y: u32,


    _last_cusor_move_time: f64,
    _currently_placing: PlacingState


}

impl BoardManagerSystem{
    pub fn new() -> BoardManagerSystem{
        BoardManagerSystem{
            cursor_x: 0,
            cursor_y: 0,
            _last_cusor_move_time: -999.0,
            _currently_placing: PlacingState::MyTeam
        }
    }

    fn create_tank(&self, ecs: &mut Ecs, xPos: u32, yPos: u32) -> EntityId{
        let tank_entity = ecs.create_entity();
        let _ = ecs.set(tank_entity, TankCmp{ is_dead: false});
        let _ = ecs.set(tank_entity, TilePosCmp{x: xPos as i32, y: yPos as i32});
        //let _ = ecs.set(tank_entity, MeshCmp{vertices: TANK_VERTICES.to_vec(), vbo: 0, shader: ShaderProgram{});

        tank_entity
    }
}

impl System for BoardManagerSystem{
    fn init(&mut self, ecs: &mut Ecs){
        // let board_manager_entity = ecs.create_entity();
        // let mut board_manager = BoardManagerCmp::new();
        // for _ in 0..5{
        //     let tank_entity = ecs.create_entity();
        //     let _ = ecs.set(tank_entity, TankCmp{ is_dead: false});
        //     let _ = ecs.set(tank_entity, TilePosCmp{x: -1, y: -1});

        //     board_manager.my_tanks.push(tank_entity);
        // }
        // for _ in 0..5{
        //     let tank_entity = ecs.create_entity();
        //     let _ = ecs.set(tank_entity, TankCmp{ is_dead: false});
        //     let _ = ecs.set(tank_entity, TilePosCmp{x: -1, y: -1});

        //     board_manager.enemy_tanks.push(tank_entity);
        // }

        // let _ = ecs.set(board_manager_entity, board_manager);
    }

    fn update(&mut self, ecs: &mut Ecs, _delta_time: f64){
        let tilemap: TilemapCmp = ecs.get(TilemapCmp::retrieve_entity(ecs).unwrap()).unwrap();
        let tile_size = tilemap.tile_size;
        let current_time: f64 = TimeCmp::retrieve(ecs).unwrap().current_time;

        let mut place_tank = false;

        //Update the cursor position
        if self._last_cusor_move_time + 0.15 < current_time
        {
            let input_sys = InputSystem::retrieve(ecs)
                .expect("Cannot retrieve the input system from the ecs, try adding the screen_data to the ecs");

            if input_sys.key_down(Key::Left) {
                self._last_cusor_move_time = current_time;
                //Because buffer underflow panics, we have to check for it
                self.cursor_x = num::clamp(self.cursor_x.checked_sub(1).unwrap_or(0), 0, tilemap.width-1);
            } if input_sys.key_down(Key::Right) {
                self._last_cusor_move_time = current_time;
                self.cursor_x = num::clamp(self.cursor_x + 1, 0, tilemap.width-1);
            } if input_sys.key_down(Key::Up) {
                //Same as key_down(Left)
                self.cursor_y = num::clamp(self.cursor_y.checked_sub(1).unwrap_or(0), 0, tilemap.height-1);
                self._last_cusor_move_time = current_time;
            } if input_sys.key_down(Key::Down) {
                self.cursor_y = num::clamp(self.cursor_y + 1, 0, tilemap.height-1);
                self._last_cusor_move_time = current_time;
            } if input_sys.key_down(Key::F) {
                self._last_cusor_move_time = current_time;
                place_tank = true;
            }
        }

        
        if place_tank {
            if self._currently_placing == PlacingState::MyTeam {
                let tank_entity = self.create_tank(ecs, self.cursor_x, self.cursor_y);
                let board_manager = BoardManagerCmp::retrieve_mut(ecs).unwrap();
                board_manager.my_tanks.push(tank_entity);
            }

            let board_manager = BoardManagerCmp::retrieve_mut(ecs).unwrap();
            
        }

        //Draw the cursor
        {
            let db_draw = DebugDraw::retrieve_mut(ecs).unwrap();
            let topleft: Vector3<f32> = Vector3::new(self.cursor_x as f32, 0.0, self.cursor_y as f32) * tile_size;
            let cursor_size: f32 = (tile_size as f32) * 0.8;
            let cursor_offset: f32 = (tile_size as f32) * 0.2;
            let blue: Vector4<f32> = Vector4::new(0.0, 0.0, 1.0, 1.0);
            db_draw.add_line(topleft + Vector3::new(cursor_offset,  0.0, cursor_offset),    topleft + Vector3::new(cursor_size,     0.0, cursor_offset), blue);
            db_draw.add_line(topleft + Vector3::new(cursor_size,    0.0, cursor_offset),    topleft + Vector3::new(cursor_size,     0.0, cursor_size), blue);
            db_draw.add_line(topleft + Vector3::new(cursor_size,    0.0, cursor_size),      topleft + Vector3::new(cursor_offset,   0.0, cursor_size), blue);
            db_draw.add_line(topleft + Vector3::new(cursor_offset,  0.0, cursor_size),      topleft + Vector3::new(cursor_offset,   0.0, cursor_offset), blue);
        }

    }

}