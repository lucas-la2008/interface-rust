use glfw::{
    Context, Glfw, GlfwReceiver, PWindow, WindowEvent
};

use std::error;

struct Renderer;

fn error_callback(error:glfw::Error, desc:String){
    panic!("GLFW error: {} description: {}", error,desc)
}

pub struct App{
    title: String,
    resolution: (u32,u32),
    loader: Renderer,
    window: PWindow,
    main_glfw: Glfw,
    event: GlfwReceiver<(f64, WindowEvent)>
}

impl App{
    pub fn new(title:&str, width: u32, height:u32) -> Self{
        let mut main_glfw = glfw::init(error_callback).unwrap();
        let (window,event) = main_glfw.create_window(width, height, &title,glfw::WindowMode::Windowed).unwrap();
        Self{
            title : String::from(title),
            resolution: (width, height),
            window,
            main_glfw,
            event,
            loader: Renderer
        }
    }

    pub fn run(&mut self){
        while !self.window.should_close(){
            self.main_glfw.poll_events();
            self.window.swap_buffers();
        }
    }
}