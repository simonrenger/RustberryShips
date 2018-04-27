use std::collections::HashMap;
use std::collections::hash_map::ValuesMut;
use std::collections::hash_map::DefaultHasher;
use std::boxed::Box;
use ::glfw;
use std::hash::{Hash, Hasher};
use std::any::TypeId;
use std::any::Any;
use std::vec;
/**
 *  Orginal source code:
 * 
struct Entity { 
    entity_id: u32,
}

impl Entity{
    
}

trait Component{
    fn start(&mut self);
    fn tick(&mut self, delta_time: f32);

    fn handle_input(&mut self, event: glfw::WindowEvent){}
}

struct EntityManager{
    entities: HashMap<u32, HashMap<u64, Box<Component>>,

    just_added: Vec<Box<Component>>,
}

impl EntityManager{
    fn new() -> EntityManager{
        EntityManager{
            entities: HashMap::new(),
        }
    }

    fn add_component<T: Component>(&mut self, entity_id: u32, t: T){
        if !self.entities.contains_key(entity_id){
            self.entities.insert(entity_id, HashMap::new());
        }

        let mut hasher = DefaultHasher::new();
        TypeId::of::<T>().hash(hasher);
        let cmp_id = hasher.finish();

        let &mut cmp_map = self.entities.get(entity_id).expect("Cannot find component list for entity, it should have been added?");
        cmp_map.insert(cmp_id, Box::new(t));
        just_added.push(Box::new(t));
    }

    fn update(&mut self, delta_time: f32){
        for component in &self.just_added{
            component.begin();
        }
        self.just_added.clear();

        for entity in self.entities.values(){
            for component in entity.values(){
                component.tick(delta_time);
            }
        }
    }
}    **/

pub trait Component : Any{
    fn start(&mut self);
    fn tick(&mut self, delta_time: f32);
    fn handle_input(&mut self, event: glfw::WindowEvent){}
}

pub struct Transform{
    x : f32,
    y : f32,
    z : f32
}
impl Transform {
    pub fn new() -> Transform{
        Transform{
            x : 0.0,
            y: 0.0,
            z: 0.0
        }
    }
}
impl Component for Transform{
    fn start(&mut self){
        println!("Hallo World start()");
    }
    fn tick(&mut self, delta_time: f32){}
}


pub struct EntityManager{
    entities : HashMap<u64,HashMap<u64,Box<Any>>>,
    tags: HashMap<String,u64>,
    ids : u64
}


impl EntityManager{
    pub fn new() ->EntityManager{
        EntityManager{
           entities: HashMap::new(),
           tags: HashMap::new(),
           ids: 0
        }
    }
    /**
     * create an entity and adds it to the list
     **/ 
    pub fn create_entity(&mut self) ->u64{
            let mut hasher = DefaultHasher::new();
            self.ids.hash(&mut hasher);
            self.ids += 1;
            let entity_id = hasher.finish();
            if !self.entities.contains_key(&entity_id){
                    self.entities.insert(entity_id, HashMap::new());
                }
            entity_id
    }

    pub fn add_tag(&mut self,entity_id : u64, name:String){
            self.tags.insert(name,entity_id);
    }

    fn get_hash<T:'static>(&mut self,t:&T)->u64{
        let mut hasher = DefaultHasher::new();
        TypeId::of::<T>().hash(&mut hasher);
        hasher.finish()
    }
     pub fn add_component<T: 'static + Component>(&mut self, entity_id: u64, t: T)
     {        
        let cmp_id = self.get_hash(&t);
        let cmp_map = self.entities.get_mut(&entity_id).expect("Cannot find component list for entity, it should have been added?");
        cmp_map.insert(cmp_id, Box::new(t));
     }
     pub fn get_by_tag(&mut self,tag:String)->Result<&mut HashMap<u64,Box<Any>>,bool>{
         if self.tags.contains_key(&tag) {
             Ok(self.entities.get_mut(&self.tags[&tag]).unwrap())
         }else{
            Err(false)
         }
     }
     pub fn get_by_id(&mut self,id : u64)->Result<&mut HashMap<u64,Box<Any>>,bool>{
         if self.entities.contains_key(&id) {
             Ok(self.entities.get_mut(&id).unwrap())
         }else{
            Err(false)
         }
     }

    pub fn get_cmp<'a,T:'static + Component>(&mut self,cmps :Result<&'a mut HashMap<u64,Box<Any>>,bool>)->Result<&'a mut T,&str>{
        match cmps{
            Ok(list)=>{
                let mut hasher = DefaultHasher::new();
                TypeId::of::<T>().hash(&mut hasher);
                let cmp_id = hasher.finish();
                if list.contains_key(&cmp_id) {
                        let result = list.get_mut(&cmp_id).unwrap();
                        Ok(result.downcast_mut::<T>().unwrap())
                }else{
                    Err("Fuck 2.0")
                }
            },
            Err(_)=>{
                Err("Fuck")
            }
        }
    }
    pub fn update(&mut self, delta_time: f32){
        for entity in self.entities.values_mut(){
            for component in entity.values_mut(){
                component.downcast_mut::<Component>().tick(delta_time);
            }
        }
    }
}