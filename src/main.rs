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

//these two for
use std::fs::File;
use std::io::BufWriter;

extern crate ply_rs;


fn main() {
    if let Err(e) = run() {
        println!("{}", debug::failure_to_string(e));
    }
}

fn save_png(gl: &gl::Gl, count: i32){
    let mut result = [0 as u8; 3*256*256];
    unsafe{
        gl.ReadPixels(0,0, 256,256, gl::RGB,gl::UNSIGNED_BYTE,result.as_ptr() as *mut std::ffi::c_void);
    }
    // println!("{:?}",result);

    let together = format!("{}{}.png", "E:/Jon/rust_opengl/out/", count);
    let path = Path::new(&together);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, 256, 256); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    
    // writer.write_image_data(&result).unwrap(); // Save

}

fn run() -> Result<(), failure::Error> {
    let mut count = 0;
    //get the resource object which contains a path
    let res = resources::Resources::from_relative_exe_path(Path::new("assets")).map_err(err_msg)?;

    //set up gl and sdl
    let sdl = sdl2::init().map_err(err_msg)?;
    let video_subsystem = sdl.video().map_err(err_msg)?;
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let window = video_subsystem
        .window("Shperical harmonics", 256, 256)
        .opengl() // add opengl flag
        .build() //.resizable()
        .unwrap();

    let _gl_context = window.gl_create_context().map_err(err_msg)?;
    let gl = gl::Gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl.Viewport(0, 0, 256, 256); // set viewport
        gl.ClearColor(0.0, 0.0, 0.0, 1.0);
    }

    // let triangle = triangle::Triangle::new(&res, &gl)?;
    let ply_model = ply_model::PlyModel::new(&res, &gl)?;


    // listen for events
    let mut event_pump = sdl.event_pump().map_err(err_msg)?;
    ply_model.setup();
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
        
        // triangle.render(&gl);
        ply_model.render(&gl,count);
        
        window.gl_swap_window();
        count +=1;
        save_png(&gl,count);
    }
    Ok(())
}

