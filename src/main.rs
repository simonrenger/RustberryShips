#[macro_use]
extern crate recs;
extern crate num;
extern crate glfw;
extern crate cgmath;
extern crate rustberry_ecs;
#[macro_use]
extern crate rustberry_ecs_derive;
extern crate tobj;

use glfw::{Action, Context, Key};

use recs::{Ecs, EntityId};

mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

mod rendering;
mod window;
mod entity_manager;
mod system_manager;
mod resource_manager;
mod engine_resources;
mod model_loader;
mod input_system;
pub mod engine_content;
mod game_content;

use gl::types::*;
use std::boxed::Box;
use std::mem;
use std::ptr;
use std::ffi::CString;
use std::cell::RefCell;
use std::path::Path;
use rendering::debug_draw::DebugDraw;
use rendering::opengl_renderer::*;
use system_manager::SystemManager;
use rustberry_ecs::EcsRetrievable;
use model_loader::load_model_from;
use engine_resources::MeshResource;

use cgmath::prelude::*;
use cgmath::{Matrix4, Vector4, Vector3, Rad, Deg, Quaternion};

use resource_manager::ResourceManager;
use engine_content::{TransformCmp, CameraCmp, MeshCmp, ScreenDataCmp, TimeCmp};
use game_content::systems::{TilemapSystem, TopDownCameraSystem, BoardManagerSystem};
use game_content::components::{TilemapCmp, TilePosCmp, BoardManagerCmp};
use std::env;

use window::Window;

// Vertex data
static VERTEX_DATA: [GLfloat; 9] = [
    0.0, 0.5, 0.0,
    0.5, -0.5, 0.0,
    -0.5, -0.5, 0.0
];



fn main() {
    let box_vertices: Vec<Vector3<f32>> = vec![
        Vector3{x: -1.0, y: -1.0, z: -1.0},
        Vector3{x: -1.0, y: -1.0, z:  1.0},
        Vector3{x: -1.0, y:  1.0, z:  1.0}, 
        Vector3{x: 1.0,  y: 1.0,  z: -1.0}, 
        Vector3{x: -1.0, y: -1.0, z: -1.0},
        Vector3{x: -1.0, y:  1.0, z: -1.0}, 
        Vector3{x: 1.0,  y: -1.0, z:  1.0},
        Vector3{x: -1.0, y: -1.0, z: -1.0},
        Vector3{x: 1.0,  y: -1.0, z: -1.0},
        Vector3{x: 1.0,  y: 1.0,  z: -1.0},
        Vector3{x: 1.0,  y: -1.0, z: -1.0},
        Vector3{x: -1.0, y: -1.0, z: -1.0},
        Vector3{x: -1.0, y: -1.0, z: -1.0},
        Vector3{x: -1.0, y:  1.0, z:  1.0},
        Vector3{x: -1.0, y:  1.0, z: -1.0},
        Vector3{x: 1.0,  y: -1.0, z:  1.0},
        Vector3{x: -1.0, y: -1.0, z:  1.0},
        Vector3{x: -1.0, y: -1.0, z: -1.0},
        Vector3{x: -1.0, y:  1.0, z:  1.0},
        Vector3{x: -1.0, y: -1.0, z:  1.0},
        Vector3{x: 1.0,  y: -1.0, z:  1.0},
        Vector3{x: 1.0,  y: 1.0,  z: 1.0 },
        Vector3{x: 1.0,  y: -1.0, z: -1.0},
        Vector3{x: 1.0,  y: 1.0,  z: -1.0},
        Vector3{x: 1.0,  y: -1.0, z: -1.0},
        Vector3{x: 1.0,  y: 1.0,  z: 1.0 },
        Vector3{x: 1.0,  y: -1.0, z:  1.0},
        Vector3{x: 1.0,  y: 1.0,  z: 1.0 },
        Vector3{x: 1.0,  y: 1.0,  z: -1.0},
        Vector3{x: -1.0, y:  1.0, z: -1.0},
        Vector3{x: 1.0,  y: 1.0,  z: 1.0 },
        Vector3{x: -1.0, y:  1.0, z: -1.0},
        Vector3{x: -1.0, y:  1.0, z:  1.0},
        Vector3{x: 1.0,  y: 1.0,  z: 1.0 },
        Vector3{x: -1.0, y:  1.0, z:  1.0},
        Vector3{x: 1.0,  y: -1.0, z:  1.0}
    ];

    let args: Vec<String> = env::args().collect();
    let do_count_fps = args.contains(&String::from("print_fps"));

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let mut mywindow = Window::new(glfw, &String::from("My own window struct!"));
    println!("window name: {}", mywindow.get_title());

    mywindow.handle.set_key_polling(true);
    mywindow.handle.make_current();

    // Create an ECS instance
    let mut ecs: Ecs = Ecs::new();
    let mut systems: SystemManager = SystemManager::new();

    gl::load_with(|s| mywindow.handle.get_proc_address(s) as *const _);

    let mut renderer = OpenglRenderer::new();
    let mut debug_renderer = DebugDraw::new();
    let mut resource_manager = RefCell::new(ResourceManager::new());
    load_resources(&mut resource_manager.borrow_mut());

    // Create GLSL shaders
    let shader_program = rendering::ShaderProgram::from_files("./resources/shaders/basic_2d.vertexshader", "./resources/shaders/basic_2d.fragshader");

    let mut vao = 0;
    let mut vbo = 0;
    let mut box_vbo = 0;
    let mut mesh_vbo = 0;

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

        // Create a buffer for cubes
        gl::GenBuffers(1, &mut box_vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, box_vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (box_vertices.len() * mem::size_of::<Vector3<f32>>()) as GLsizeiptr,
            mem::transmute(&box_vertices[0]),
            gl::STATIC_DRAW,
        );

        // Create a buffer for the mesh
        let mesh_vertices = resource_manager.borrow().get::<MeshResource>("TankModel").expect("TankModel is not here!").calculate_vertices();
        gl::GenBuffers(1, &mut mesh_vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, mesh_vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (mesh_vertices.len() * mem::size_of::<Vector3<f32>>()) as GLsizeiptr,
            mem::transmute(&mesh_vertices[0]),
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

        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
    }

    // Add entities to the ECS
    let camera_entity: EntityId = ecs.create_entity();
    let _ = ecs.set(camera_entity, TransformCmp{
        position: Vector3{x: 30.0, y: 15.0, z: 30.0}, 
        orientation: Quaternion::one(),
        scale: Vector3{x: 1.0, y: 1.0, z: 1.0},
    });
    let _ = ecs.set(camera_entity, CameraCmp{
        fovy: 90.0,
        aspect: mywindow.aspect_ratio(),
        near: 0.1,
        far: 1000.0,
    });
    
    let box_entity: EntityId = ecs.create_entity();
    let _ = ecs.set(box_entity, TransformCmp{
        position: Vector3{x: 2.0, y: 1.0, z: 0.0}, 
        orientation: Quaternion::one(),
        scale: Vector3{x: 1.0, y: 1.0, z: 1.0},
    });
    let _ = ecs.set(box_entity, MeshCmp{
        vertices: box_vertices.clone(),
        vbo: box_vbo,
        shader: shader_program.clone(),
    });

    let mesh_entity: EntityId = ecs.create_entity();
    let _ = ecs.set(mesh_entity, TransformCmp{
        position: Vector3{x: 12.0, y: 1.0, z: 0.0}, 
        orientation: Quaternion::one(),
        scale: Vector3{x: 1.0, y: 1.0, z: 1.0},
    });
    let _ = ecs.set(mesh_entity, MeshCmp{
        vertices: resource_manager.borrow().get::<MeshResource>("TankModel").expect("TankModel is not here!").calculate_vertices(),
        vbo: mesh_vbo,
        shader: shader_program.clone(),
    });

    let tilemap_entity: EntityId = ecs.create_entity();
    let _ = ecs.set(tilemap_entity, TilemapCmp{width: 6, height: 6, tile_size: 10.0});
    let _ = ecs.set(tilemap_entity, BoardManagerCmp::new());

    //An entity to dump misc singleton components onto
    let dump_entity: EntityId = ecs.create_entity();
    let _ = ecs.set(dump_entity, ScreenDataCmp{
        mywindow: mywindow,
    });
    let _ = ecs.set(dump_entity, debug_renderer);
    let _ = ecs.set(dump_entity, TimeCmp{current_time: 0.0, delta_time: 0.0});
    
    systems.add(Box::new(TopDownCameraSystem::new(-45.0)));
    systems.add(Box::new(engine_content::FreelookCameraSystem{movement_speed: 10.5, rotation_speed: 70.5, active: false}));
    systems.add(Box::new(TilemapSystem{}));
    systems.add(Box::new(BoardManagerSystem::new()));

    systems.init(&mut ecs);

    let mut last_time = glfw.get_time();
    let mut should_close: bool = false;
    while !should_close {
        let mut window_events: Vec<glfw::WindowEvent> = Vec::new();
        {
            let wd = ScreenDataCmp::retrieve_mut(&mut ecs).unwrap();
            should_close = wd.mywindow.handle.should_close();

            glfw.poll_events();
            let events = glfw::flush_messages(&wd.mywindow.events);
            for (_, event) in events {
                window_events.push(event.clone());
                handle_window_event(&mut wd.mywindow.handle, event.clone());
            }
        }

        for event in window_events {
            systems.handle_event(&mut ecs, &event);
        }

        //Update systems:
        let current_time = glfw.get_time();
        let dt = current_time - last_time;
        last_time = current_time;
        if let Some(time_entity) = TimeCmp::retrieve_entity(&ecs){
            let mut time_cmp: TimeCmp = ecs.get(time_entity).unwrap();
            time_cmp.current_time = current_time;
            time_cmp.delta_time = dt;
            let _ = ecs.set(time_entity, time_cmp);
        }
        if do_count_fps {
            println!("FPS: {:.6} | Frame time: {:.6} ms", 1.0/dt, dt);
        }

        systems.update(&mut ecs, dt);



        //Add some lines:
        // let green_color = Vector4{x: 0.0, y: 1.0, z: 0.0, w: 0.0};
        // debug_renderer.add_line(Vector3{x: 4.0, y: 1.0, z: 0.0}, Vector3{x: -4.0, y: -1.0, z: 0.0}, green_color.clone());
        // debug_renderer.add_line(Vector3{x: -4.0, y: -1.0, z: 0.0}, Vector3{x: 2.0, y: 0.8, z: 0.0}, green_color.clone());
        // debug_renderer.add_line(Vector3{x: 0.4, y: 0.8, z: 0.0}, Vector3{x: 4.0, y: 1.0, z: 0.0}, green_color.clone());

        let camera_transform: TransformCmp = ecs.get(camera_entity).unwrap();
        let camera_component: CameraCmp = ecs.get(camera_entity).unwrap();
        let perspective_mat = cgmath::perspective(Rad::from(Deg(camera_component.fovy)), camera_component.aspect, camera_component.near, camera_component.far);
        let view_mat = camera_transform.calculate_model_matrix().invert().expect("Cannot invert view matrix!");
        let model_mat = Matrix4::identity();
        let mvp = perspective_mat * view_mat * model_mat;

        unsafe {
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

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
        renderer.render(&camera_transform, &camera_component, &mut ecs);
        {
            let db_renderer = DebugDraw::retrieve_mut(&mut ecs).unwrap();
            db_renderer.render(&vp);
            db_renderer.clear();
        }

        ScreenDataCmp::retrieve_mut(&mut ecs).unwrap().mywindow.handle.swap_buffers();
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
        // glfw::WindowEvent::Key(Key::A, _, Action::Repeat, _) => {
        //     println!("Pressed the a key!" );
        //     let mut camera_transform: TransformCmp = ecs.get(camera_entity).unwrap();
        //     camera_transform.position.x += 0.1;
        //     ecs.set(camera_entity, camera_transform.clone());
        // }
        // glfw::WindowEvent::Key(Key::D, _, Action::Repeat, _) => {
        //     println!("Pressed the d key!" );
        //     let mut camera_transform: TransformCmp = ecs.get(camera_entity).unwrap();
        //     camera_transform.position.x -= 0.1;
        //     ecs.set(camera_entity, camera_transform.clone());
        // }
        _ => {}
    }
}

fn load_resources(resource_manager: &mut ResourceManager) {
    resource_manager.add::<MeshResource>("TankModel", load_model_from(&Path::new("./resources/models/Low_poly_rusty_tank.obj")));
    resource_manager.get_mut::<MeshResource>("TankModel").unwrap().scale = 0.01;
}
