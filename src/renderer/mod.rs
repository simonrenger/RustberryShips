// use super::*;
// use std::sync::mpsc::Receiver

// pub struct Renderer{
// 	pub window: glfw::Window,

// 	//TODO: Make this a seperate struct
// 	pub events: Receiver<(f64, WindowEvent)>
// }

// impl Renderer {
// 	pub fn new(_glfw: &mut glfw::Glfw) -> Renderer{
// 		let (mut window, events) = _glfw.create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
//         .expect("Failed to create GLFW window.");

// 		Renderer{
// 			window: window,
// 			events: events,
// 		}
// 	}

// 	fn initialize(&mut self){

// 	}

// 	fn draw_world(&mut self){

// 	}
// }

// impl Drop for Renderer {
// 	fn drop(&mut self) {

// 	}
// }