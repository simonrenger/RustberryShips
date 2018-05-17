use system_manager::System;
use recs::Ecs;
use recs;
use ::*;
use super::components::TilemapCmp;

pub struct TilemapSystem{

}

impl System for TilemapSystem{
    fn init(&mut self){
        println!("TilemapSystem is initialized!");
    }
    fn update(&mut self, ecs: &mut Ecs, delta_time: f64){
        //Get the tilemap
        let filter = component_filter!(TilemapCmp);
        let mut entity_ids: Vec<EntityId> = Vec::new();
        ecs.collect_with(&filter, &mut entity_ids);
        if entity_ids.is_empty() {
            println!("Cannot find a TilemapCmp in the scene! please add one!");
            return ();
        }
        let tilemap: TilemapCmp = ecs.get(entity_ids[0]).unwrap();
        
        //Visualise the tilemap
        for x in 0..tilemap.width{
            for y in 0..tilemap.height{
                
            }
        }
    }
}