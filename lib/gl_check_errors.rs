pub enum GlStatus{
    CompilationError(String),
    LinkError(String),
    NoError
}

pub fn check_glshader_compile_error(shader_id: gl::types::GLuint)-> GlStatus{
    let mut status: gl::types::GLint = 0;

    unsafe {
        //Get the compilation status (GL_COMPILE_STATUS)
        gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut status);
        
        //if result is GL_FALSE, the compilation failed
        if status == (gl::FALSE as i32) {
            
            let mut log_length: gl::types::GLint = 0;
            
            // Discover the error message length
            gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut log_length);
    
            if log_length <= 1{
                return GlStatus::CompilationError("Shader compilation failed with no Log\n".to_string());
            }
            
            // Create a buffer to store the error message
            let mut error_log: Vec<gl::types::GLchar> = Vec::with_capacity(log_length as usize);
            gl::GetShaderInfoLog(shader_id, log_length, &mut log_length, error_log.as_mut_ptr());
            let message:String = error_log.iter().map(|&c| c as u8 as char).collect();
            // returns the error message and a GlStatus variant in a tuple
            GlStatus::CompilationError(message)
        } else {
            GlStatus::NoError
        }
    }
}

pub fn check_glprogram_link_error(program_id: gl::types::GLuint) -> GlStatus{
    let mut status: gl::types::GLint = 0;

    unsafe{
        //Get the linking status (gl::LINK_STATUS)
        gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut status);
        
        //if result is GL_FALSE, the linking failed
        if status == (gl::FALSE as i32) {
            let mut log_length: gl::types::GLint = 0;
            // Discover the error message length
            gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut log_length);

            if log_length <= 1{
                return GlStatus::LinkError("Shader linking failed with no Log".to_string());
            }
            
            // Create a buffer to store the error message
            let mut error_log = Vec::<gl::types::GLchar>::with_capacity(log_length as usize);
            gl::GetProgramInfoLog(program_id, log_length, &mut log_length, error_log.as_mut_ptr());
            // Converts the i8 vector into a string
            let message = error_log.iter().map(|&ch| ch as u8 as char).collect::<String>();
            
            // returns the error message and a GlStatus variant in a tuple
            GlStatus::LinkError(message)
        }else {
            GlStatus::NoError
        }
    }
}
