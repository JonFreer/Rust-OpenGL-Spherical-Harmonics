use gl;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::AddAssign;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]



pub struct f32_f32_f32 {
    pub d0: f32,
    pub d1: f32,
    pub d2: f32,
}

impl f32_f32_f32 {
    pub fn new(d0: f32, d1: f32, d2: f32) -> f32_f32_f32 {
        f32_f32_f32 {
            d0, d1, d2
        }
    }

    pub unsafe fn vertex_attrib_pointer(gl: &gl::Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            3, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid
        );
    }

    pub fn normalize(&mut self){

        let unit = (self.d0*self.d0 + self.d1 * self.d1 + self.d2 * self.d2).sqrt();
        *self =  Self{
            d0: self.d0 / unit,
            d1: self.d1 / unit ,
            d2: self.d2 / unit 
        }
    }

}

impl From<(f32, f32, f32)> for f32_f32_f32 { //alows casting from tuple
    fn from(other: (f32, f32, f32)) -> Self {
        f32_f32_f32::new(other.0, other.1, other.2)
    }
}

impl Sub for f32_f32_f32{
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output{
        Self{
            d0:self.d0-other.d0, 
            d1:self.d1-other.d1, 
            d2:self.d2 -other.d2
        }
    }
}

impl Mul for f32_f32_f32{
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output{
        Self{
            d0: self.d1 * other.d2 - self.d2 * other.d1,
            d1: self.d2 * other.d0 - self.d0 * other.d2,
            d2: self.d0 * other.d1 - self.d1 * other.d0
        }
    }
}

impl AddAssign for f32_f32_f32{
    fn add_assign(&mut self, other: Self){
        *self = Self {
            d0: self.d0 + other.d0,
            d1: self.d1 + other.d1,
            d2: self.d2 + other.d2
        }
    }
}