use nalgebra_glm::{
    Vec2,
};

use crate::style::Style;

pub trait BaseObject{
    fn new()-> Self where Self: Sized;
    
    fn get_size(&self)-> Vec2;
    fn get_position(&self) -> Vec2;
    fn get_style(&self) -> &Style;
    
    fn set_position(&mut self, x:f32,y:f32);
    fn set_size(&mut self, x:f32,y:f32);
    fn set_color(&mut self, x:f32,y:f32,z:f32,w:f32);
    fn set_border_color(&mut self, x:f32,y:f32,z:f32,w:f32);
    fn set_border_size(&mut self, size: f32);
    fn set_visibility(&mut self, visibility: bool);
}