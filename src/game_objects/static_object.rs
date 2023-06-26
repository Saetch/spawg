//these objects are meant to be used as the background or anything static, without any logic attached to it

use crate::rendering::{vertex::Vertex, sprites::sprite_mapping::Sprite};


pub struct StaticObject{
    pub(crate) texture: Sprite,
    pub position: (f64, f64),
    pub vertices: [Vertex; 4],
    id: u64,
}

impl StaticObject {
    pub(crate) fn new(texture: Sprite, position: (f64, f64), vertices: [Vertex; 4]) -> Self{
        Self{
            texture,
            position,
            vertices,
            id: 0,
        }
    }

    pub(crate) fn get_id(&self) -> u64{
        self.id
    }

    pub(crate) fn set_id(&mut self, id: u64){
        self.id = id;
    }
}