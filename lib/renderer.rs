mod renderer_init;

use nalgebra_glm as glm;
use glm::Mat4;

use crate::elements::base_object::BaseObject;

#[allow(unused)]
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

    pub fn render(&mut self){
        if !self.init_complete{
            panic!("Renderer initialization incomplete");
        }
    
        if self.quad_vbo == 0 || self.vao== 0 || self.data_vbo == 0{
            panic!("Vertices buffer not initialized");
        }
        unsafe{
        gl::ClearColor(0.0f32,0.0f32,0.0f32,0.0f32);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        gl::UseProgram(self.shader_program);
        
            gl::BindVertexArray(self.vao);
        
            gl::DrawArraysInstanced(
                gl::TRIANGLES,
                0,
                6,
                self.elements_count as i32
            );
        
            gl::UseProgram(0);
        }
    }

    pub fn load(&mut self, elements: &Vec<Box<dyn BaseObject>> ){
        let mut temp: Vec<ElementData> = Vec::new();
        temp.reserve(elements.len());
        for element in elements{
            
            let style = element.get_style();
            if !style.visible{
                continue;
            }
            
            let position = element.get_position();
            let size = element.get_size();
            
            temp.push(ElementData{
                size_x: size.x,
                size_y:size.y,
                position_x: position.x,
                position_y: position.y,
                color_x: style.color.x,
                color_y: style.color.y,
                color_z: style.color.z,
                color_a: style.color.w,
                border_color_x: style.border_color.x,
                border_color_y: style.border_color.y,
                border_color_z: style.border_color.z,
                border_color_a: style.border_color.w,
                border_size: style.border_size
            });
            self.elements_count += 1;
        }
        unsafe{
            gl::BindBuffer(gl::ARRAY_BUFFER, self.data_vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (temp.len() * core::mem::size_of::<ElementData>()) as isize,
                temp.as_ptr().cast(),
                gl::DYNAMIC_DRAW
            );
        }
    }
}
