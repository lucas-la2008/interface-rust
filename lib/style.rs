
use nalgebra_glm::Vec4;
pub struct Style{
    pub color: Vec4,
    pub border_color: Vec4,
    pub border_size: f32,
    pub visible: bool,
}

impl Style{
    pub fn new() -> Self{
        Self{
            color: Vec4::zeros(),
            border_color: Vec4::zeros(),
            border_size: 0.0,
            visible: true
        }
    }
}