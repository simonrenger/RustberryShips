extern crate glfw;
extern crate cgmath;

use glfw::{Action, Context, Key};

mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

mod rendering;
mod window;
mod engine_content;

use gl::types::*;
use std::mem;
use std::ptr;
use std::ffi::CString;

use cgmath::prelude::*;
use cgmath::{Matrix4, Vector4, Vector3, Rad, Deg, Quaternion};

use engine_content::{TransformCmp, CameraCmp};

use window::Window;

// Vertex data
static VERTEX_DATA: [GLfloat; 9] = [
    0.0, 0.5, 0.0,
    0.5, -0.5, 0.0,
    -0.5, -0.5, 0.0
    ];

//===================
// Shader code:
//===================
// static VS_SRC: &'static str = "
// #version 150
// in vec2 position;
// void main() {
//     gl_Position = vec4(position, 0.0, 1.0);
// }";

// static FS_SRC: &'static str = "
// #version 150
// out vec4 out_color;
// void main() {
//     out_color = vec4(1.0, 1.0, 1.0, 1.0);
// }";

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let mut mywindow = Window::new(glfw, &String::from("My own window struct!"));
    println!("window name: {}", mywindow.get_title());

    // let (mut window, events) = glfw.create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
    //     .expect("Failed to create GLFW window.");

    //let mut r = renderer::Renderer::new(&mut glfw);
    //let &mut window = &mut r.window;
    //let &events = &r.events;

    // &mut window = r.window;
    mywindow.handle.set_key_polling(true);
    mywindow.handle.make_current();

    gl::load_with(|s| mywindow.handle.get_proc_address(s) as *const _);

    let mut debug_renderer = rendering::debug_draw::DebugDraw::new();

    // Create GLSL shaders
    //let shader_program = rendering::ShaderProgram::from_source(VS_SRC, FS_SRC);
    let shader_program = rendering::ShaderProgram::from_files("./resources/shaders/basic_2d.vertexshader", "./resources/shaders/basic_2d.fragshader");

    let mut vao = 0;
    let mut vbo = 0;

    unsafe {
        // Create Vertex Array Object
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (VERTEX_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            mem::transmute(&VERTEX_DATA[0]),
            gl::STATIC_DRAW,
        );

        // Use shader program
        gl::UseProgram(shader_program.handle());
        gl::BindFragDataLocation(shader_program.handle(), 0, CString::new("out_color").unwrap().as_ptr());

        // Specify the layout of the vertex data
        let pos_attr = gl::GetAttribLocation(shader_program.handle(), CString::new("position").unwrap().as_ptr());
        gl::EnableVertexAttribArray(pos_attr as GLuint);
        gl::VertexAttribPointer(
            pos_attr as GLuint,
            3,
            gl::FLOAT,
            gl::FALSE as GLboolean,
            0,
            ptr::null(),
        );
    }

    let camera_transform = TransformCmp{
        position: Vector3{x: 0.0, y: 0.0, z: 2.0}, 
        orientation: Quaternion::<f32>::one(),
        scale: Vector3{x: 1.0, y: 1.0, z: 1.0}
    };
    let camera_component = CameraCmp{
        fovy: 90.0,
        aspect: mywindow.aspect_ratio(),
        near: 0.1,
        far: 100.0,
    };
    
    
    while !mywindow.handle.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&mywindow.events) {
            handle_window_event(&mut mywindow.handle, event);
        }

        //Add some lines:
        let green_color = Vector4{x: 0.0, y: 1.0, z: 0.0, w: 0.0};
        debug_renderer.add_line(Vector3{x: 4.0, y: 1.0, z: 0.0}, Vector3{x: -4.0, y: -1.0, z: 0.0}, green_color.clone());
        debug_renderer.add_line(Vector3{x: -4.0, y: -1.0, z: 0.0}, Vector3{x: 2.0, y: 0.8, z: 0.0}, green_color.clone());
        debug_renderer.add_line(Vector3{x: 0.4, y: 0.8, z: 0.0}, Vector3{x: 4.0, y: 1.0, z: 0.0}, green_color.clone());

        let perspective_mat = cgmath::perspective(Rad::from(Deg(camera_component.fovy)), camera_component.aspect, camera_component.near, camera_component.far);
        let view_mat = camera_transform.calculate_model_matrix().invert().expect("Cannot invert view matrix!");
        let model_mat = Matrix4::from_value(1.0);
        let mvp = perspective_mat * view_mat * model_mat;

        unsafe {
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            // Use shader program
            gl::UseProgram(shader_program.handle());

            // Specify the layout of the vertex data
            let pos_attr = gl::GetAttribLocation(shader_program.handle(), CString::new("position").unwrap().as_ptr());
            gl::EnableVertexAttribArray(pos_attr as GLuint);
            gl::VertexAttribPointer(
                pos_attr as GLuint,
                3,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                0,
                ptr::null(),
            );
            

            let mvp_location = gl::GetUniformLocation(shader_program.handle(), CString::new("u_MVP").unwrap().as_ptr());
            gl::UniformMatrix4fv(mvp_location, 1, gl::FALSE, &mvp[0][0]);

            // Draw a triangle from the 3 vertices
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
        let vp = perspective_mat * view_mat;
        debug_renderer.render(&vp);

        mywindow.handle.swap_buffers();
    }

    // Cleanup
    unsafe {
        // gl::DeleteProgram(program);
        // gl::DeleteShader(fs);
        // gl::DeleteShader(vs);
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteVertexArrays(1, &vao);
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        glfw::WindowEvent::Key(Key::A, _, Action::Press, _) => {
            println!("Pressed the a key!" )
        }
        _ => {}
    }
}