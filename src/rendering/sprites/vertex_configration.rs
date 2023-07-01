#![allow(dead_code, non_camel_case_types)]

use crate::rendering::vertex::Vertex;
#[derive(Debug, Clone, Copy)]
pub(crate) enum VertexConfigration {
    SQUARE_SMALL_1,
    NEARLY_SQUARE_RECTANGLE_0,

    LINE_HORIZONTAL,
    LINE_VERTICAL,
    SOMETHINGSOMETHING,
    
    ELSE
} 

pub trait VertexConfigrationTrait {

    fn get_vertices(&self) -> [Vertex; 4];
}

impl VertexConfigrationTrait for VertexConfigration {
    #[inline(always)]
    fn get_vertices(&self) -> [Vertex; 4] {
        match self {
            VertexConfigration::SQUARE_SMALL_1 => SQUARE_1_1_VERTEX_CONF(),
            VertexConfigration::SOMETHINGSOMETHING => todo!(),
            VertexConfigration::ELSE => todo!(),
            VertexConfigration::LINE_HORIZONTAL => SMALL_LINE_HORIZONTAL_VERTEX_CONF(),
            VertexConfigration::LINE_VERTICAL => SMALL_LINE_VERTICAL_VERTEX_CONF(),
            VertexConfigration::NEARLY_SQUARE_RECTANGLE_0 => SQUARE_0_0_VERTEX_CONF(),
            _ => todo!(),
        }
    }
}

#[allow(non_snake_case)]      
#[inline(always)]
fn SQUARE_1_1_VERTEX_CONF()->  [Vertex; 4]{
   centered_rectangle(6.0, 6.0)
}

#[allow(non_snake_case)]
#[inline(always)]
fn SQUARE_0_0_VERTEX_CONF()->  [Vertex; 4]{
   centered_rectangle(0.48, 0.48)
}

#[allow(non_snake_case)]      
#[inline(always)]
fn SMALL_LINE_HORIZONTAL_VERTEX_CONF()->  [Vertex; 4]{
    centered_rectangle(0.5, 0.06581)
}

#[allow(non_snake_case)]      
#[inline(always)]
fn SMALL_LINE_VERTICAL_VERTEX_CONF()->  [Vertex; 4]{
    centered_rectangle(0.06, 0.5)
}

#[inline(always)]
fn centered_rectangle(width:  f32, height: f32) -> [Vertex; 4] {
    [
        Vertex { position: [width / 2.0, -height / 2.0], tex_coords: [1.0, 1.0]}, // A
        Vertex { position: [width / 2.0, height / 2.0], tex_coords: [1.0, 0.0]}, // B
        Vertex { position: [-width / 2.0, height / 2.0], tex_coords: [0.0, 0.0] }, // C
        Vertex { position: [-width / 2.0, -height / 2.0], tex_coords: [0.0, 1.0] }, // D
    ]
}




