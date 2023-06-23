//these objects are meant to be used as the background or anything static, without any logic attached to it

use crate::rendering::{vertex::Vertex, sprites::sprite_mapping::Sprite};


pub struct StaticObject{
    pub(crate) texture: Sprite,
    pub position: (f64, f64),
    pub vertices: [Vertex; 4]
}

impl StaticObject {
    pub(crate) fn new(texture: Sprite, position: (f64, f64), vertices: [Vertex; 4]) -> Self{
        Self{
            texture,
            position,
            vertices,
        }
    }
}