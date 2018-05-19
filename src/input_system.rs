use ::recs::{Ecs, EntityId};
use ::glfw::{Window, Key};
use ::glfw;
use engine_content::ScreenDataCmp;
use ::*;

#[allow(dead_code)]
pub struct InputSystem<'a> {
    pub window: &'a Window,
}

impl<'a> InputSystem<'a>{
    pub fn retrieve(ecs: &'a mut Ecs) -> Option<InputSystem<'a>>{
        let mut screen_data_ids: Vec<EntityId> = Vec::new();
        let screen_data_filter = component_filter!(ScreenDataCmp);
        ecs.collect_with(&screen_data_filter, &mut screen_data_ids);
        if screen_data_ids.is_empty() {
            return None{};
        }
        let screen_data: &ScreenDataCmp = ecs.borrow(screen_data_ids[0]).expect("We querried with ecs.collect_with but it is not there?");
        let window = &screen_data.mywindow.handle;

        Some(InputSystem{
            window: window,
        })
    }

    pub fn key_down(&self, key: Key) -> bool{
        let state = self.window.get_key(key);
        
        state == glfw::Action::Press
    }

    pub fn key_up(&self, key: Key) -> bool{
        let state = self.window.get_key(key);
        
        state == glfw::Action::Release
    }

    pub fn key_state(&self, key: Key) -> glfw::Action{
        self.window.get_key(key)
    }
}