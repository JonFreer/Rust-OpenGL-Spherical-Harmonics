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
use std::env;
use rand::Rng;
//these two for
use std::fs::File;
use std::io::BufWriter;

extern crate ply_rs;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt,Debug)]
#[structopt(rename_all = "kebab-case")]
struct Cli {

  /// Set speed
    // we don't want to name it "speed", need to look smart
    #[structopt(short = "i", long = "in")]
    input: std::path::PathBuf,

    #[structopt(short = "o", long = "out")]
    output: std::path::PathBuf,

    #[structopt(short = "n", long = "name")]
    name: String,

    #[structopt(short = "h", long = "harmonic",default_value = "1.0")]
    harmonic: String,

    #[structopt(short = "s", long = "sequence",default_value = "1.0")]
    sequence: i32,

    #[structopt(short = "f", long = "frequency" ,default_value = "1.0")]
    freq: i32,

}


fn main() {
    if let Err(e) = run() {
        println!("{}", debug::failure_to_string(e));
    }
}


fn save_png(gl: &gl::Gl, count: i32, output: &std::path::PathBuf,name:&String){
    let mut result = [0 as u8; 3*256*256];

    unsafe{
        gl.ReadPixels(0,0, 256,256, gl::RGB,gl::UNSIGNED_BYTE,result.as_ptr() as *mut std::ffi::c_void);
    }
    let mut path = output.clone();
    path.push(format!("{}{:0>5}.png",name,count.to_string()));
    let file = File::create(path.as_path()).unwrap();
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, 256, 256); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    
    writer.write_image_data(&result).unwrap(); // Save

}

fn run() -> Result<(), failure::Error> {
    let args = Cli::from_args();
    println!("{:?}", args);

    let mut count = -1;
    let mut rng = rand::thread_rng();

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
    let ply_model = ply_model::PlyModel::new(&res, &gl,args.input)?;


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
        
        // triangle.render(&gl);

        if 1==args.sequence{
            ply_model.render(&gl,count);
            
            window.gl_swap_window();
            if count>=0{
                save_png(&gl,count,&args.output, &args.name);
            }
            

            count += rng.gen_range(1..=args.freq);
            
            if count>=32768{ //exit program when number of sample reached
                break 'main
            }
        }else{
            
            ply_model.render_single(&gl,&args.harmonic);
            window.gl_swap_window();
            save_png(&gl,0,&args.output, &args.name);
            
            if count>=3{
                break 'main
            }

            count += 1;
        }
    }
    Ok(())
}

