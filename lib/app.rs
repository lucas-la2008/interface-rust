use std::ptr::null;

use glfw::{
    Context,
    Glfw,
    GlfwReceiver,
    PWindow,
    Window,
    WindowEvent,
    ffi::{
        glfwGetWindowUserPointer
    }
};

use super::renderer::Renderer;

fn error_callback(error:glfw::Error, desc:String){
    panic!("GLFW error: {} description: {}", error,desc)
}

pub struct App{
    title: String,
    width: u32,
    height: u32,
    loader: Renderer,
    main_glfw: Glfw,
    main_window: PWindow,
    event: GlfwReceiver<(f64, WindowEvent)>
}

impl App{
    pub fn new(title:&str, width: u32, height:u32) -> Self{
        let mut main_glfw = glfw::init(error_callback).unwrap();
        let (mut main_window,event) = main_glfw.create_window(width, height, &title,glfw::WindowMode::Windowed).unwrap();
        
        gl::load_with(|procname| {
            main_window.get_proc_address(procname)
                .map_or(std::ptr::null(), |c| c as *const () as *const std::ffi::c_void)
        });
        
        let mut loader = Renderer::new();
        loader.init();
        main_window.set_framebuffer_size_callback(App::size_callback);
        
        Self{
            title : String::from(title),
            width,
            height,
            main_window,
            main_glfw,
            event,
            loader
        }
    }

    pub fn run(&mut self){
        while !self.main_window.should_close(){
            self.main_glfw.poll_events();
            self.main_window.swap_buffers();
        }
    }

    fn size_callback(window: &mut Window, width:i32, height:i32){
        let ptr;
        unsafe {
            ptr = glfwGetWindowUserPointer(window.window_ptr());
        }
        if unsafe{ptr.is_null()} {
            panic!("Could not use the window pointer");
        }
        let mut app = ptr as *mut App;
        // app->onResize(width, height);
    }

}