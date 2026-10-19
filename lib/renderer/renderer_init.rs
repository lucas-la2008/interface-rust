use super::{ELEMENT_DATA_SIZE, Renderer};
use crate::shaders::*;
use crate::gl_check_errors::*;
use std::{ffi::c_void, mem::size_of};

impl Renderer {
    pub fn init(&mut self) {
        unsafe {
            gl::GenBuffers(1, &mut self.quad_vbo);
            gl::GenBuffers(1, &mut self.data_vbo);
            gl::GenVertexArrays(1, &mut self.vao);
            gl::Disable(gl::CULL_FACE);
            gl::BindVertexArray(self.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.quad_vbo);

            gl::VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE,
                (2 * size_of::<f32>()) as i32,
                0 as *const c_void
            );
            gl::EnableVertexAttribArray(0);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.data_vbo);

            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                ELEMENT_DATA_SIZE,
                0 as *const c_void,
            );
            gl::EnableVertexAttribArray(1);

            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                ELEMENT_DATA_SIZE,
                (2 * ELEMENT_DATA_SIZE) as *const c_void,
            );
            gl::EnableVertexAttribArray(2);

            gl::VertexAttribPointer(
                3,
                4,
                gl::FLOAT,
                gl::FALSE,
                ELEMENT_DATA_SIZE,
                (4 * ELEMENT_DATA_SIZE) as *const c_void,
            );
            gl::EnableVertexAttribArray(3);

            gl::VertexAttribPointer(
                4,
                4,
                gl::FLOAT,
                gl::FALSE,
                ELEMENT_DATA_SIZE,
                (8 * ELEMENT_DATA_SIZE) as *const c_void,
            );
            gl::EnableVertexAttribArray(4);

            gl::VertexAttribPointer(
                5,
                1,
                gl::FLOAT,
                gl::FALSE,
                ELEMENT_DATA_SIZE,
                (12 * ELEMENT_DATA_SIZE) as *const c_void,
            );
            gl::EnableVertexAttribArray(5);

            gl::VertexAttribDivisor(1, 1);
            gl::VertexAttribDivisor(2, 1);
            gl::VertexAttribDivisor(3, 1);
            gl::VertexAttribDivisor(4, 1);
            gl::VertexAttribDivisor(5, 1);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.quad_vbo);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                size_of_val(&self.quad_vertices) as isize,
                self.quad_vertices.as_ptr() as *const c_void,
                gl::DYNAMIC_DRAW,
            );
            gl::BindVertexArray(0);

            self.vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(self.vertex_shader, 1, &VERTEX_SHADER_SOURCE.as_bytes().as_ptr().cast(), &(VERTEX_SHADER_SOURCE.len() as i32));
            gl::CompileShader(self.vertex_shader);
            match check_glshader_compile_error(self.vertex_shader){
                
                GlStatus::CompilationError(message) => {
                    panic!("Compilation error: {message}")
                }

                _ =>{}
            }
            
            self.fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(self.fragment_shader, 1, &FRAGMENT_SHADER_SOURCE.as_bytes().as_ptr().cast(), &(FRAGMENT_SHADER_SOURCE.len() as i32));
            gl::CompileShader(self.fragment_shader);
            match check_glshader_compile_error(self.fragment_shader){
                
                GlStatus::CompilationError(message) => {
                    panic!("Compilation error: {message}")
                }

                _ =>{}
            }
            
            self.shader_program = gl::CreateProgram();
            gl::AttachShader(self.shader_program, self.vertex_shader);
            gl::AttachShader(self.shader_program, self.fragment_shader);
            gl::LinkProgram(self.shader_program);
            match check_glprogram_link_error(self.shader_program){
                
                GlStatus::LinkError(message) => {
                    panic!("Linking error: {message}")
                }

                _ =>{}
            }
            gl::DeleteShader(self.vertex_shader);
            gl::DeleteShader(self.fragment_shader);

            self.projection_location =
                gl::GetUniformLocation(self.shader_program, c"projection".as_ptr());
            gl::UseProgram(self.shader_program);

            self.init_complete = true;
        }
    }
}
