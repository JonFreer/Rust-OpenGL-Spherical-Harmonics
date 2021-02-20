#[macro_use] extern crate failure;

extern crate gl;
extern crate sdl2;

pub mod render_gl;

pub mod resources;
// use crate::resources::Resources;
use std::path::Path;
use failure::err_msg;


fn main() {
    if let Err(e) = run() {
        println!("{}", failure_to_string(e));
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

    

    //load shaders into program
    let shader_program = render_gl::Program::from_res(&gl, &res, "shaders/triangle").unwrap();

    // tell program to use shaders
    shader_program.set_used();

    //define vertexes
    let vertices: Vec<f32> = vec![
        // positions      // colors
        0.5, -0.5, 0.0, 1.0, 0.0, 0.0, // bottom right
        -0.5, -0.5, 0.0, 0.0, 1.0, 0.0, // bottom left
        0.0, 0.5, 0.0, 0.0, 0.0, 1.0, // top
    ];

    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        //vbo is the vbo ID
        gl.GenBuffers(1, &mut vbo);
    }

    // Set array buffer size and type
    unsafe {
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl.BufferData(
            gl::ARRAY_BUFFER, // target (buffer type)
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW, // usage: Switch to DYNAMIC_DRAW for changing model http://docs.gl/gl4/glBufferData
        );
        gl.BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer
    }

    // Create the VAO, with ID of 0
    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl.GenVertexArrays(1, &mut vao);
    }
    unsafe {
        gl.BindVertexArray(vao); //make it current by binding it

        gl.BindBuffer(gl::ARRAY_BUFFER, vbo); //bind VBO again
        gl.EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
        gl.VertexAttribPointer(
            0,         // index of the generic vertex attribute ("layout (location = 0)")
            3,         // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null(),                                     // offset of the first component
        );

        gl.EnableVertexAttribArray(1); // this is "layout (location = 0)" in vertex shader
        gl.VertexAttribPointer(
            1, // index of the generic vertex attribute ("layout (location = 0)") : really location = 1
            3, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid // offset of the first component
        );


        gl.BindBuffer(gl::ARRAY_BUFFER, 0); //Unbind VBO and VAO as not needed
        gl.BindVertexArray(0);
    }

    // list for events
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

        shader_program.set_used();
        unsafe {
            gl.BindVertexArray(vao);
            gl.DrawArrays(
                gl::TRIANGLES, // mode
                0,             // starting index in the enabled arrays
                3,             // number of indices to be rendered
            );
        }

        window.gl_swap_window();
    }
    Ok(())
}

pub fn failure_to_string(e: failure::Error) -> String {
    use std::fmt::Write;

    let mut result = String::new();

    for (i, cause) in e.iter_chain().collect::<Vec<_>>().into_iter().rev().enumerate() {
        if i > 0 {
            let _ = writeln!(&mut result, "   Which caused the following issue:");
        }
        let _ = write!(&mut result, "{}", cause);
        if let Some(backtrace) = cause.backtrace() {
            let backtrace_str = format!("{}", backtrace);
            if backtrace_str.len() > 0 {
                let _ = writeln!(&mut result, " This happened at {}", backtrace);
            } else {
                let _ = writeln!(&mut result);
            }
        } else {
            let _ = writeln!(&mut result);
        }
    }

    result
}