mod renderer_init;

use glm::Mat4;
use nalgebra_glm as glm;

pub struct ElementData {
    size_x: f32,
    size_y: f32,
    position_x: f32,
    position_y: f32,
    color_x: f32,
    color_y: f32,
    color_z: f32,
    color_a: f32,
    border_color_x: f32,
    border_color_y: f32,
    border_color_z: f32,
    border_color_a: f32,
    border_size: f32,
}

pub const ELEMENT_DATA_SIZE: i32 = size_of::<ElementData>() as i32;

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
            projection: glm::ortho(0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32),
            quad_vbo: 0,
            data_vbo: 0,
            vao: 0,
            vertex_shader: 0,
            fragment_shader: 0,
            shader_program: 0,
            projection_location: 0,
            elements_count: 0,
            quad_vertices: [
                0.0f32, 0.0f32, 1.0f32, 0.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 0.0f32, 1.0f32,
                0.0f32, 0.0f32,
            ],
            init_complete: false,
        }
    }

    pub fn resize(&mut self, width: i32, height:i32){
        self.projection = glm::ortho(0.0f32, 1920.0f32, 1080.0f32, 0.0f32, 0.0f32, 0.0f32);
        unsafe {
            gl::Viewport(0,0,width,height);
        
            if self.shader_program != 0{
                gl::UseProgram(self.shader_program);
                gl::UniformMatrix4fv(
                    self.projection_location,
                    1,
                    gl::FALSE,
                    self.projection.as_ptr()
                );
            }
        }
    }

}
