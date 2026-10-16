use nalgebra_glm as glm;
use glm::Mat4;
use std::{
    ffi::c_void,
    mem::size_of
};

struct ElementData{
    size_x:f32,
    size_y:f32,
    position_x:f32,
    position_y:f32,
    color_x:f32,
    color_y:f32,
    color_z:f32,
    color_a:f32,
    border_color_x:f32,
    border_color_y:f32,
    border_color_z:f32,
    border_color_a:f32,
    border_size:f32,
}


pub struct Renderer {
    quad_vbo: u32,
    data_vbo: u32,

    vao: u32,

    vertex_shader: u32,
    fragment_shader: u32,
    shader_program: u32,
    projection_location: i32,

    elements_count: usize,

    quad_vertices: [f32; 12],

    projection: Mat4,

    init_complete: bool,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            projection: glm::ortho(0.0f32, 1920.0f32, 1080.0f32, 0.0f32, 0.0f32, 0.0f32),
            quad_vbo: 0,
            data_vbo: 0,
            vao: 0,
            vertex_shader: 0,
            fragment_shader: 0,
            shader_program: 0,
            projection_location: 0,
            elements_count: 0,
            quad_vertices: [
                0.0f32, 0.0f32,
                1.0f32, 0.0f32,
                1.0f32, 1.0f32,

                1.0f32, 1.0f32,
                0.0f32, 1.0f32,
                0.0f32, 0.0f32,
            ],
            init_complete: false
        }
    }
    pub fn init(&mut self){
        unsafe {
            gl::GenBuffers(1, &mut self.quad_vbo);
            gl::GenBuffers(1, &mut self.data_vbo);
            gl::GenVertexArrays(1, &mut self.vao);
            gl::Disable(gl::CULL_FACE);
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.quad_vbo);

            gl::VertexAttribPointer(0,
                2,
                gl::FLOAT,
                gl::FALSE,
                0,
                0 as *const c_void
            );
            gl::EnableVertexAttribArray(0);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.data_vbo);

            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                 0, //size_of<ElementData>(),
                0 as *const c_void
            );
        }
    }
}
