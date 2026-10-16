use nalgebra_glm::{
    Mat4,
    
};

use nalgebra_glm as glm;
struct Renderer{
    quad_vbo: u32,
    data_vbo: u32,

    Vao: u32,
    
    vertex_shader: u32,
    fragment_shader: u32,
    shader_program: u32,
    projection_location: i32,

    elements_count: usize,
    
    quad_vertices: [f32;12],

    projection: Mat4 = glm::ortho(
        0.0f32,
        1920.0f32,
        1080.0f32,
        0.0f32,
        0.f32,
        0.f32
    ),

    bool initComplete = false,

}