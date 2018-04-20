use std::sync::mpsc::Receiver;
use ::glfw;

pub struct Window{
	title: String,
    _glfw: glfw::Glfw,
    pub handle: glfw::Window,
    pub events: Receiver<(f64, glfw::WindowEvent)>,
    pub w: f32,
    pub h: f32,
}

impl Window {
    //TODO: Add resolution params
	pub fn new(mut _glfw: glfw::Glfw, title: &String) -> Window{
		let title_copy = title.clone();

        let (window, events) = _glfw.create_window(800, 600, title, glfw::WindowMode::Windowed).expect("Failed to create GLFW window.");

		Window{
            title: title_copy,
            _glfw: _glfw,
            handle: window,
            events: events,
            w: 800.0,
            h: 600.0,
            }
	}

	pub fn get_title<'a>(&'a self) -> &'a String{
		&self.title
	}

    pub fn aspect_ratio(&self) -> f32{
        self.w / self.h
    }
}
