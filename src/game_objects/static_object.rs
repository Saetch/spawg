//these objects are meant to be used as the background or anything static, without any logic attached to it

use crate::rendering::vertex::Vertex;


pub struct StaticObject{
    pub texture_id: u16,
    pub position: (f64, f64),
}

impl StaticObject {
    pub fn construct_vertices(&self, _camera_position: (f32, f32)) -> [crate::rendering::vertex::Vertex; 6]{
        todo!();
    }
}