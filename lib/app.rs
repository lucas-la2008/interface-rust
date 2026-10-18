
use glfw::{
    Context,
    Glfw,
    GlfwReceiver,
    PWindow,
    Window,
    WindowEvent,
};

use crate::elements::base_object::BaseObject;

use super::renderer::Renderer;

fn error_callback(error:glfw::Error, desc:String){
    panic!("GLFW error: {} description: {}", error,desc)
}

static mut APP_PTR: usize = 0;

#[allow(unused)]
pub struct App{
    title: String,
    width: u32,
    height: u32,
    loader: Renderer,
    main_glfw: Glfw,
    main_window: PWindow,
    event: GlfwReceiver<(f64, WindowEvent)>,
    children_elements: Vec<Box<dyn BaseObject>>
}

impl App{
    pub fn new(title:&str, width: u32, height:u32) -> Box<Self>{
        let mut main_glfw = glfw::init(error_callback).unwrap();
        let (main_window,event) = main_glfw.create_window(width, height, &title,glfw::WindowMode::Windowed).unwrap();
        
        let loader = Renderer::new();
        
        Box::new(Self{
            title : String::from(title),
            width,
            height,
            main_window,
            main_glfw,
            event,
            loader,
            children_elements: Vec::new()
        })
    }

    pub fn config(&mut self){
        self.main_window.make_current();
        gl::load_with(|procname| {
            self.main_window.get_proc_address(procname)
                .map_or(std::ptr::null(), |c| c as *const () as *const std::ffi::c_void)
        });
        self.loader.init();
        unsafe {
            APP_PTR = self as *mut App as usize;
        }
        self.main_window.set_framebuffer_size_callback(App::size_callback);
    }

    pub fn create_element<T: BaseObject + 'static>(&mut self) -> &mut dyn BaseObject{
        let temp = Box::new(T::new());
        self.children_elements.push(temp);
        self.children_elements.last_mut().unwrap().as_mut()
    }
    
    pub fn run(&mut self){
        while !self.main_window.should_close(){
            self.loader.load(&self.children_elements);
            self.loader.render();
            self.main_glfw.poll_events();
            self.main_window.swap_buffers();
        }
    }

    fn size_callback(_window: &mut Window, width:i32, height:i32){
        let ptr: usize;
        unsafe {
            ptr = APP_PTR;
        }
        if ptr == 0 {
            panic!("Could not use the window pointer");
        }
        let app = ptr as *mut App;
        unsafe {(*app).on_resize(width, height);}
    }

    fn on_resize(&mut self,width: i32, height: i32)
    {
        self.width = width as u32;
        self.height = height as u32;
        self.loader.resize(width, height);
    }
}