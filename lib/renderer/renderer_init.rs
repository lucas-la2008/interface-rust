use super::{
    Renderer,
    ELEMENT_DATA_SIZE,
    ElementData
};
use std::{
    ffi::c_void,
    mem::size_of,
    ptr::null
};
use crate::shaders::*;


impl Renderer{
    pub fn init(&mut self) {
    unsafe {
        gl::GenBuffers(1, &mut self.quad_vbo);
        gl::GenBuffers(1, &mut self.data_vbo);
        gl::GenVertexArrays(1, &mut self.vao);
        gl::Disable(gl::CULL_FACE);
        gl::BindVertexArray(self.vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, self.quad_vbo);

        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 0, 0 as *const c_void);
        gl::EnableVertexAttribArray(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, self.data_vbo);

        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            size_of::<ElementData>() as i32,
            0 as *const c_void,
        );
        gl::EnableVertexAttribArray(1);

        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            size_of::<ElementData>() as i32,
            (2 * ELEMENT_DATA_SIZE) as *const c_void,
        );
        gl::EnableVertexAttribArray(2);

        gl::VertexAttribPointer(
            3,
            4,
            gl::FLOAT,
            gl::FALSE,
            size_of::<ElementData>() as i32,
            (4 * ELEMENT_DATA_SIZE) as *const c_void,
        );
        gl::EnableVertexAttribArray(3);

        gl::VertexAttribPointer(
            4,
            4,
            gl::FLOAT,
            gl::FALSE,
            size_of::<ElementData>() as i32,
            (8 * ELEMENT_DATA_SIZE) as *const c_void,
        );
        gl::EnableVertexAttribArray(4);

        gl::VertexAttribPointer(
            5,
            1,
            gl::FLOAT,
            gl::FALSE,
            size_of::<ElementData>() as i32,
            (12 * ELEMENT_DATA_SIZE) as *const c_void,
        );
        gl::EnableVertexAttribArray(5);

        gl::VertexAttribDivisor(1, 1);
        gl::VertexAttribDivisor(2, 1);
        gl::VertexAttribDivisor(3, 1);
        gl::VertexAttribDivisor(4, 1);
        gl::VertexAttribDivisor(5, 1);

        gl::BindBuffer(gl::ARRAY_BUFFER, self.data_vbo);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            size_of_val(&self.quad_vertices) as isize,
            self.quad_vertices.as_ptr() as *const c_void,
            gl::DYNAMIC_DRAW,
        );
        gl::BindVertexArray(0);

        self.vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(gl::VERTEX_SHADER, 1, VERTEX_SHADER_PTR_PTR, null());
        gl::CompileShader(self.vertex_shader);

        self.fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(gl::FRAGMENT_SHADER, 1, FRAGMENT_SHADER_PTR_PTR, null());
        gl::CompileShader(self.fragment_shader);

        self.shader_program = gl::CreateProgram();
        gl::AttachShader(self.shader_program, self.vertex_shader);
        gl::AttachShader(self.shader_program, self.fragment_shader);
        gl::LinkProgram(self.shader_program);

        gl::DeleteShader(self.vertex_shader);
        gl::DeleteShader(self.fragment_shader);

        self.projection_location =
            gl::GetUniformLocation(self.shader_program, c"projection".as_ptr());
        gl::UseProgram(self.shader_program);

        self.init_complete = true;
    }
}
}