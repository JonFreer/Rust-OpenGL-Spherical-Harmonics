
use crate::render_gl::{self, buffer, data};
use crate::resources::Resources;
use failure;
use gl;
use ply_rs as ply;

#[derive(VertexAttribPointers, Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = 0]
    pos: data::f32_f32_f32,
    #[location = 1]
    clr: data::f32_f32_f32,
}

pub struct PlyModel {
    program: render_gl::Program,
    vbo: buffer::ArrayBuffer,
    vao: buffer::VertexArray,
    ebo: buffer::ElementBuffer,
}

struct Face{
    points:(u32,u32,u32)
}

impl PlyModel {
    pub fn new(res: &Resources, gl: &gl::Gl) -> Result<PlyModel, failure::Error> {
        //load shaders into program
        let program = render_gl::Program::from_res(&gl, &res, "shaders/triangle")?;
  
        // set up a reader, in this case a file.
        let path = "assets/models/res.ply";
        let mut f = std::fs::File::open(path).unwrap();

        // create a parser
        let p = ply::parser::Parser::<ply::ply::DefaultElement>::new();

        // use the parser: read the entire file
        let ply = p.read_ply(&mut f);

        // make sure it did work
        assert!(ply.is_ok());
        let ply = ply.unwrap();

        // proof that data has been read
        println!("Ply header: {:#?}", ply.header);
        println!("Ply data: {:#?}", ply.payload["face"][0]["vertex_indices"]);

        // Load veticies 
        let mut vertices = Vec::<Vertex>::new();

        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        let mut r = 0.0;
        let mut g = 0.0;
        let mut b = 0.0;

        for f in &ply.payload["vertex"] {
            for item in f{
                match (item.0.as_str(), item.1) {
                    ("x", ply::ply::Property::Float(v)) => x = *v,
                    ("y", ply::ply::Property::Float(v)) => y = *v,
                    ("z", ply::ply::Property::Float(v)) => z = *v,
                    ("red", ply::ply::Property::UChar(v)) => r = *v as f32,
                    ("green", ply::ply::Property::UChar(v)) => g = *v as f32,
                    ("blue", ply::ply::Property::UChar(v)) => b = *v as f32,
                    ("alpha", _) =>(),
                    (k, _) => panic!("Vertex: Unexpected key/value combination: key: {}", k),
                }
            }
            vertices.push(Vertex{
                pos:(x,y,z).into(),
                clr: (r/255.0,g/255.0,b/255.0).into()
            });
            
        }
      

        //Load Faces 
        let mut faces = Vec::<Face>::new();
        for f in &ply.payload["face"] {
            let mut a = 0;
            let mut b = 0;
            let mut c = 0;
            match &f["vertex_indices"] {
                ply::ply::Property::ListInt(v) => {a = v[0];b= v[1];c = v[2]},
                _ => ()

            }
            // let ply::ply::Property::ListInt(v) = &f["vertex_indices"];
            faces.push(Face{points:(a as u32,b as u32,c as u32)});
        }

        //create and load data into VBO
        let vbo = buffer::ArrayBuffer::new(&gl);
        vbo.bind();
        vbo.static_draw_data(&vertices);
        vbo.unbind();


        //create and load data into EBO
        let ebo = buffer::ElementBuffer::new(&gl);
        ebo.bind();
        ebo.static_draw_data(&faces);
        ebo.unbind();

        // Create the VAO, with ID of 0

        let vao = buffer::VertexArray::new(&gl);

        vao.bind();
        vbo.bind();
        Vertex::vertex_attrib_pointers(&gl); //set up VAO
        vbo.unbind();
        vao.unbind();

        Ok(PlyModel{
            program,
            vbo,
            vao,
            ebo,
        })
    }

    pub fn render(&self, gl: &gl::Gl) {
        //renders triagnle based on loaded data
        // tell program to use shaders
        self.program.set_used();
        self.vao.bind();
        self.ebo.bind();
        unsafe {
            
            // gl.DrawArrays(
            //     gl::TRIANGLES, // mode
            //     0,             // starting index in the enabled arrays
            //     6*80000,             // number of indices to be rendered
            // );data.as_ptr() as *const gl::types::GLvoid
            gl.DrawElements(gl::TRIANGLES,800000,gl::UNSIGNED_INT,0 as *const gl::types::GLvoid);
        }
    }
}
