use ::recs::{Ecs, EntityId};
use std::vec::Vec;
use std::boxed::Box;
use ::glfw;

pub trait System{
    fn init(&mut self);
    fn update(&mut self, ecs: &mut Ecs, delta_time: f64);
    fn handle_event(&mut self, event: &glfw::WindowEvent){}
}

pub struct SystemManager{
    systems: Vec<Box<System>>,
    just_added_systems: Vec<usize>,
}

impl SystemManager{
    pub fn new() -> SystemManager{
        SystemManager{
            systems: Vec::new(),
            just_added_systems: Vec::new(),
        }
    }

    ///Currently doens't do anything but maybe later :)
    pub fn init(&mut self){
        
    }

    pub fn update(&mut self, ecs: &mut Ecs, delta_time: f64){
        for i in 0..self.just_added_systems.len(){
            let system_index = self.just_added_systems[i];
            self.systems[system_index].init();            
        }
        self.just_added_systems.clear();

        for i in 0..self.systems.len(){
            self.systems[i].update(ecs, delta_time);
        }
        // for just_added in self.just_added_systems{
        //     self.systems[just_added].init();
        // }
        // self.just_added_systems.clear();

        // for mut system in self.systems{
        //     system.update(ecs, delta_time);
        // }
    }

    pub fn handle_event(&mut self, ecs: &mut Ecs, event: &glfw::WindowEvent){
        let system_amount = self.systems.len();
        for i in 0..system_amount{
            self.systems[i].handle_event(event);
        }
        // for mut system in self.systems{
        //     system.handle_event(window, event);
        // }
    }

    pub fn add(&mut self, system: Box<System>){
        self.systems.push(system);
        self.just_added_systems.push(self.systems.len() - 1);
    }

    //TODO: Simon pls Implement :P
    pub fn remove(&mut self, index: u32){
        //Make sure to also update the just_added_systems array indices
    }
}