// use std::collections::HashMap;
// use std::collections::hash_map::ValuesMut;
// use std::collections::hash_map::DefaultHasher;
// use std::boxed::Box;
// use ::glfw;
// use std::hash::{Hash, Hasher};
// use std::any::TypeId;
// use std::any::Any;
// use std::vec;
// use std::ops::DerefMut;

// /**
//  *  Orginal source code:
//  * 
// struct Entity { 
//     entity_id: u32,
// }

// impl Entity{
    
// }

// trait Component{
//     fn start(&mut self);
//     fn tick(&mut self, delta_time: f32);

//     fn handle_input(&mut self, event: glfw::WindowEvent){}
// }

// struct EntityManager{
//     entities: HashMap<u32, HashMap<u64, Box<Component>>,

//     just_added: Vec<Box<Component>>,
// }

// impl EntityManager{
//     fn new() -> EntityManager{
//         EntityManager{
//             entities: HashMap::new(),
//         }
//     }

//     fn add_component<T: Component>(&mut self, entity_id: u32, t: T){
//         if !self.entities.contains_key(entity_id){
//             self.entities.insert(entity_id, HashMap::new());
//         }

//         let mut hasher = DefaultHasher::new();
//         TypeId::of::<T>().hash(hasher);
//         let cmp_id = hasher.finish();

//         let &mut cmp_map = self.entities.get(entity_id).expect("Cannot find component list for entity, it should have been added?");
//         cmp_map.insert(cmp_id, Box::new(t));
//         just_added.push(Box::new(t));
//     }

//     fn update(&mut self, delta_time: f32){
//         for component in &self.just_added{
//             component.begin();
//         }
//         self.just_added.clear();

//         for entity in self.entities.values(){
//             for component in entity.values(){
//                 component.tick(delta_time);
//             }
//         }
//     }
// }    **/

// static component_id_counter: u64 = 0;

// pub trait AnyCastable : Any{
//     fn as_any<'a>(&'a self) -> &'a Any;
//     fn as_any_mut<'a>(&'a mut self) -> &'a mut Any;
// }

// impl<T: Any> AnyCastable for T{
//     fn as_any<'a>(&'a self) -> &'a Any{
//         self
//     }

//     fn as_any_mut<'a>(&'a mut self) -> &'a mut Any{
//         self
//     }
// }

// pub trait Component : AnyCastable{
//     fn start(&mut self);
//     fn tick(&mut self, delta_time: f32);
//     fn handle_input(&mut self, event: glfw::WindowEvent){}
// }

// pub struct Transform{
//     x : f32,
//     y : f32,
//     z : f32
// }
// impl Transform {
//     pub fn new() -> Transform{
//         Transform{
//             x : 0.0,
//             y: 0.0,
//             z: 0.0
//         }
//     }
// }
// impl Component for Transform{
//     fn start(&mut self){
//         println!("Hallo World start()");
//     }
//     fn tick(&mut self, delta_time: f32){}
// }

// type EntityId = u32;

// pub struct EntityManager{
//     entities : HashMap<EntityId,HashMap<u64,Box<Any>>>,
//     tags: HashMap<String,EntityId>,
//     ids : u64,
//     id_counter: EntityId,

//     just_added_entities: Vec<EntityId>,
// }

// fn get_default_hash<T: 'static>(t: &T)->u64{
//     let mut hasher = DefaultHasher::new();
//     TypeId::of::<T>().hash(&mut hasher);
//     hasher.finish()
// }

// fn get_default_hash_of<T: 'static>()->u64{
//     let mut hasher = DefaultHasher::new();
//     TypeId::of::<T>().hash(&mut hasher);
//     hasher.finish()
// }

// impl EntityManager{
//     pub fn new() -> EntityManager{
//         EntityManager{
//            entities: HashMap::new(),
//            tags: HashMap::new(),
//            ids: 0,
//            id_counter: 0,
//            just_added_entities: Vec::new()
//         }
//     }
//     /**
//      * create an entity and adds it to the list
//      **/ 
//     pub fn create_entity(& self) ->EntityId{
//         let entity_id = self.generate_entity_id();
//         if !self.entities.contains_key(&entity_id){
//             self.entities.insert(entity_id, HashMap::new());
//         }
//         entity_id
//     }

//     pub fn add_tag(&mut self,entity_id : EntityId, name:String){
//         self.tags.insert(name,entity_id);
//     }

//     pub fn add_component<'a, T: Component>(&'a mut self, entity_id: EntityId, t: T)
//     {
//         let cmp_id = get_default_hash(&t);
//         let cmp_map = self.entities.get_mut(&entity_id).expect("Cannot find component list for entity, it should have been added?");
//         cmp_map.insert(cmp_id, Box::new(t));
//     }
//     pub fn get_by_tag(&mut self, tag:String)->Result<&mut HashMap<u64,Box<Any>>,bool>{
//         if self.tags.contains_key(&tag) {
//             Ok(self.entities.get_mut(&self.tags[&tag]).unwrap())
//         }else{
//             Err(false)
//         }
//     }

//     pub fn get_single_cmp<'a, 'b, T: Component>(&'a mut self, entity_id: &'b EntityId) -> Option<&'a mut T>{
//         if self.entities.contains_key(entity_id) {
//             let component_id = get_default_hash_of::<T>();
//             let components = self.entities.get_mut(entity_id);
//             if components.unwrap().contains_key(&component_id) {
//                 return Some(components.unwrap().get_mut(&component_id).unwrap().as_any().downcast_mut().unwrap());
//             }
//         }

//         None
//     }

//     pub fn get_cmp<'a, T:'static + Component>(&'a self, cmps: Result<&'a mut HashMap<u64,Box<Component>>,bool>) -> Result<&'a mut T,&str>{
//         match cmps{
//             Ok(list)=>{
//                 let cmp_id = get_default_hash_of::<T>();
//                 if list.contains_key(&cmp_id) {
//                         let result = list.get_mut(&cmp_id).unwrap();
//                         let cmp: &'a mut T = result.as_any_mut().downcast_mut().unwrap();
//                         Ok(cmp)
//                         //Ok(result.downcast::<T>().unwrap())
//                 }else{
//                     Err("Fuck 2.0")
//                 }
//             },
//             Err(_)=>{
//                 Err("Fuck")
//             }
//         }
//     }
//     pub fn update(&mut self, delta_time: f32){
//         for entity in self.entities.values_mut(){
//             for component in entity.values_mut(){
//                 component.get_mut().downcast_mut::<Component>().tick(delta_time);
//             }
//         }
//     }

//     //===================
//     // Private methods:
//     //===================
//     /// Return all the components of an entity with 'id'
//     pub fn get_components_by_id(&mut self, id: EntityId)->Result<&mut HashMap<u64, Box<Component>>,bool>{
//         if self.entities.contains_key(&id) {
//             Ok(self.entities.get_mut(&id).unwrap())
//         }else{
//             Err(false)
//         }
//     }

//     fn generate_entity_id(&mut self) -> EntityId{
//         self.id_counter += 1;
        
//         self.id_counter
//     }
// }