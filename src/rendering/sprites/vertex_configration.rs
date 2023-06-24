#![allow(dead_code, non_camel_case_types)]

use crate::rendering::vertex::Vertex;
#[derive(Debug)]
pub(crate) enum VertexConfigration {
    SQUARE_1_1,
    SOMETHINGSOMETHING,
    ELSE
} 

trait VertexConfigrationTrait {
    fn get_vertex_configration(&self) -> [Vertex; 4];
}

impl VertexConfigrationTrait for VertexConfigration {
    fn get_vertex_configration(&self) ->  [Vertex; 4]{
        match self {
            VertexConfigration::SQUARE_1_1 => SQUARE_1_1_VERTEX_CONF(),
            VertexConfigration::SOMETHINGSOMETHING => todo!(),
            VertexConfigration::ELSE => todo!(),
        }
    }
}


const fn SQUARE_1_1_VERTEX_CONF()->  [Vertex; 4]{
   [
    Vertex { position: [0.5, -0.5], tex_coords: [1.0, 1.0]}, // A
    Vertex { position: [0.5, 0.5], tex_coords: [1.0, 0.0]}, // B
    Vertex { position: [-0.5, 0.5], tex_coords: [0.0, 0.0] }, // C
    Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 1.0] }, // D
   ]
}

