
use crate::render_gl::{self, buffer, data};
use crate::resources::Resources;
use failure;
use gl;

#[derive(VertexAttribPointers, Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = 0]
    pos: data::f32_f32_f32,
    #[location = 1]
    clr: data::f32_f32_f32,
}

pub struct Triangle {
    program: render_gl::Program,
    vbo: buffer::ArrayBuffer,
    vao: buffer::VertexArray,
}

impl Triangle {
    pub fn new(res: &Resources, gl: &gl::Gl) -> Result<Triangle, failure::Error> {
        //load shaders into program
        let program = render_gl::Program::from_res(&gl, &res, "shaders/triangle")?;
  

        //define vertexes
        let vertices: Vec<Vertex> = vec![
            Vertex {
                pos: (0.5, -0.5, 0.0).into(),
                clr: (1.0, 0.0, 0.0).into(),
            }, // bottom right
            Vertex {
                pos: (-0.5, -0.5, 0.0).into(),
                clr: (0.0, 1.0, 0.0).into(),
            }, // bottom left
            Vertex {
                pos: (0.0, 0.5, 0.0).into(),
                clr: (0.0, 0.0, 1.0).into(),
            }, // top
            Vertex {
                pos: (0.5, 0.5, 0.0).into(),
                clr: (1.0, 1.0, 1.0).into(),
            },
            Vertex {
                pos: (0.5, 0.1, 0.0).into(),
                clr: (0.0, 0.0, 1.0).into(),
            },
            Vertex {
                pos: (0.2, 0.1, 0.0).into(),
                clr: (0.0, 0.0, 1.0).into(),
            },
        ];

        //create and load data into VBO
        let vbo = buffer::ArrayBuffer::new(&gl);
        vbo.bind();
        vbo.static_draw_data(&vertices);
        vbo.unbind();

        // Create the VAO, with ID of 0

        let vao = buffer::VertexArray::new(&gl);

        vao.bind();
        vbo.bind();
        Vertex::vertex_attrib_pointers(&gl); //set up VAO
        vbo.unbind();
        vao.unbind();

        Ok(Triangle{
            program,
            vbo:vbo,
            vao,
        })
    }

    pub fn render(&self, gl: &gl::Gl) {
        //renders triagnle based on loaded data
        // tell program to use shaders
        self.program.set_used();
        self.vao.bind();
        unsafe {
            
            gl.DrawArrays(
                gl::TRIANGLES, // mode
                0,             // starting index in the enabled arrays
                6,             // number of indices to be rendered
            );
        }
    }
}
