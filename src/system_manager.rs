use ::recs::{Ecs};
use std::vec::Vec;
use std::boxed::Box;
use std::cell::RefCell;
use ::resource_manager::ResourceManager;
use ::glfw;

pub trait System{
    fn init(&mut self, ecs: &mut Ecs);
    fn update(&mut self, ecs: &mut Ecs, delta_time: f64);
    fn handle_event(&mut self, _event: &glfw::WindowEvent){}

    // fn get_world(&self) -> &World;
}

//Idk if this gets used but I wanted to make it
#[macro_export]
macro_rules! impl_property {
    ( $x:expr, $y:expr ) => {
        pub fn set_$x(&mut self, new: $y) {
            self.$x = new;
        }

        pub fn get_$x(&mut self) -> $y {
            self.$x
        }
    };
}

//A collection of services like ResourceManager
pub struct World<'a>{
    _resource_manager_ref: Option<&'a RefCell<ResourceManager<'a>>>
}

impl<'a> World<'a>{
    fn new() -> World<'a>{
        World{
            _resource_manager_ref: None,
        }
    }
}

pub struct SystemManager<'a>{
    systems: Vec<Box<System>>,
    just_added_systems: Vec<usize>,

    _world: World<'a>,
}

impl<'a> SystemManager<'a>{
    pub fn new() -> SystemManager<'a>{
        SystemManager{
            systems: Vec::new(),
            just_added_systems: Vec::new(),
            _world: World::new(),
        }
    }

    ///Currently doens't do anything but maybe later :)
    pub fn init(&mut self, ecs: &mut Ecs){
        // if let Some(rcm_ref) = self._resource_manager_ref{
        //     for i in 0..self.systems.len(){
        //         //self.systems[i].set_resource_manager_ref(rcm_ref);
        //     }
        // }
    }

    pub fn update(&mut self, ecs: &mut Ecs, delta_time: f64){
        for i in 0..self.just_added_systems.len(){
            let system_index = self.just_added_systems[i];
            self.systems[system_index].init(ecs);            
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
