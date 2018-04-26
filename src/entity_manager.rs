use std::collections::HashMap;
use std::boxed::Box;
use ::glfw;
use std::collections::hash_map::DefaultHasher;
use std::any::TypeId;
use std::vec::Vec;

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
}
