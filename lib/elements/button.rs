use nalgebra_glm::Vec2;
use super::base_object::BaseObject;
use crate::style::Style;

pub struct Button{
    position: Vec2,
    size: Vec2,
    style: Style,
}

impl BaseObject for Button{
    fn new()-> Self{
        Self{
            position: Vec2::zeros(),
            size: Vec2::zeros(),
            style: Style::new()
        }
    }
    fn get_position(&self) -> Vec2 {
        let mut temp: Vec2 = Vec2::zeros();
        temp.x = self.position.x;
        temp.y = self.position.y;
        temp
    }

    fn get_size(&self)-> Vec2 {
        let mut temp: Vec2 = Vec2::zeros();
        temp.x = self.size.x;
        temp.y = self.size.y;
        temp
    }

    fn get_style(&self) -> &Style {
        &self.style
    }
    
    fn set_position(&mut self, x:f32,y:f32){
        self.position.x = x;
        self.position.y = y;
    }

    fn set_size(&mut self, x:f32,y:f32) {
        self.size.x = x;
        self.size.y = y;
    }

    fn set_color(&mut self, x:f32,y:f32,z:f32,w:f32){
        self.style.color.x = x;
        self.style.color.y = y;
        self.style.color.z = z;
        self.style.color.w = w;
    }

    fn set_border_color(&mut self, x:f32,y:f32,z:f32,w:f32){
        self.style.border_color.x = x;
        self.style.border_color.y = y;
        self.style.border_color.z = z;
        self.style.border_color.w = w;
    }

    fn set_border_size(&mut self, size: f32){
        self.style.border_size = size;
    }

    fn set_visibility(&mut self, visibility: bool){
        self.style.visible = visibility;
    }
}