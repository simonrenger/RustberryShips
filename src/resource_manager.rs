/**
 * A global manager for assets
 * 
 */

use std::collections::HashMap;
use std::string::String;
use std::boxed::Box;
use std::any::{Any, TypeId};

pub trait Resource : Any {
    fn as_any(&self) -> &Any;
}

impl<T: Any> Resource for T {
    fn as_any(&self) -> &Any{
        self
    }
}


pub struct ResourceManager<'a>{
    _resources: HashMap<String, Box<Any>>,
            _dwmjka: Option<&'a i32>,
}

impl<'a> ResourceManager<'a>{
    pub fn new() -> ResourceManager<'a>{
        ResourceManager{
            _resources: HashMap::new(),
            _dwmjka: None,
        }
    }

    //TODO: Make it return a Result so we can give an error message
    pub fn get<T: Resource>(&self, name: &str) -> Option<&T>{
        //TODO: Check for type
        match self._resources.get(&String::from(name)){
            Some(resource) =>{ 
                // //It's not unsafe STFU compiler
                // unsafe{
                //     let resource_ref: &Resource = resource;
                //     let resource_ptr = resource_ref as *const _;
                //     Some(&*(resource_ptr as *const T))
                // }
                Some(resource.downcast_ref::<T>().expect("The resource somehow doesn't cast down"))
            },
            None => None,
        }
    }

    pub fn get_mut<T: Resource>(&mut self, name: &str) -> Option<&mut T>{
        //TODO: Check for type
        match self._resources.get_mut(&String::from(name)){
            Some(resource) =>{ 
                // //It's not unsafe STFU compiler
                // unsafe{
                //     let resource_ref: &Resource = resource;
                //     let resource_ptr = resource_ref as *const _;
                //     Some(&*(resource_ptr as *const T))
                // }
                
                Some(resource.downcast_mut::<T>().expect("The resource somehow doesn't cast down"))
            },
            None => None,
        }
    }

    pub fn add<T: Resource>(&mut self, name: &str, resource: T) {
        self._resources.insert(String::from(name), Box::new(resource));
    }
}
