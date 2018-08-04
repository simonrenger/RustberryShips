use rustberry_ecs::EcsRetrievable;
use rustberry_ecs::{Ecs, EntityId};
use ::*;

#[derive(Clone, PartialEq, Debug)]
pub struct TankCmp{
    pub is_dead: bool,
}

#[derive(Clone, PartialEq, Debug, EcsRetrievable)]
pub struct TilemapCmp{
    pub width: u32,
    pub height: u32,
    pub tile_size: f32,
}

#[derive(Clone, PartialEq, Debug)]
pub struct TilePosCmp{
    pub x: i32,
    pub y: i32,
}

impl TilePosCmp{
    pub fn from_vector(v: Vector3<f32>) -> TilePosCmp{
        TilePosCmp{x: v.x.round() as i32, y: v.z.round() as i32}
    }
}

#[derive(Clone, PartialEq, Debug, EcsRetrievable)]
pub struct BoardManagerCmp{
    pub my_tanks: Vec<EntityId>,
    pub enemy_tanks: Vec<EntityId>,
}

impl BoardManagerCmp{
    pub fn new() -> BoardManagerCmp{
        BoardManagerCmp{
            my_tanks: Vec::new(),
            enemy_tanks: Vec::new(),
        }
    }
}