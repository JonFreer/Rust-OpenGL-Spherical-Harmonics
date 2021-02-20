#[macro_use] extern crate failure;
#[macro_use] extern crate render_gl_derive;
extern crate gl;
extern crate sdl2;



pub mod render_gl;
mod triangle;
mod ply_model;
pub mod debug;
pub mod resources;
// use crate::resources::Resources;
use std::path::Path;
use failure::err_msg;
use render_gl::data;
use render_gl::buffer;
extern crate ply_rs;


fn main() {
    if let Err(e) = run() {
        println!("{}", debug::failure_to_string(e));
    }
}


fn run() -> Result<(), failure::Error> {
    //get the resource object which contains a path
    let res = resources::Resources::from_relative_exe_path(Path::new("assets")).map_err(err_msg)?;

    //set up gl and sdl
    let sdl = sdl2::init().map_err(err_msg)?;
    let video_subsystem = sdl.video().map_err(err_msg)?;
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let window = video_subsystem
        .window("Game", 900, 700)
        .opengl() // add opengl flag
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().map_err(err_msg)?;
    let gl = gl::Gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl.Viewport(0, 0, 900, 700); // set viewport
        gl.ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let triangle = triangle::Triangle::new(&res, &gl)?;
    let plyModel = ply_model::PlyModel::new(&res, &gl)?;


    // listen for events
    let mut event_pump = sdl.event_pump().map_err(err_msg)?;
    
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT);
        }

        triangle.render(&gl);
        plyModel.render(&gl);
        window.gl_swap_window();
    }
    Ok(())
}

