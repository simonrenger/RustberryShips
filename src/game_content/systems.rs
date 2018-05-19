use system_manager::System;
use recs::Ecs;
use ::*;
use rendering::debug_draw::DebugDraw;
use super::components::TilemapCmp;
use rustberry_ecs::EcsRetrievable;
use ::cgmath::{Vector3};

pub struct TilemapSystem{

}

impl System for TilemapSystem{
    fn init(&mut self){
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