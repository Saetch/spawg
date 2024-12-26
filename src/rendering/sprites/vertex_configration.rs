#![allow(dead_code, non_camel_case_types)]

use crate::rendering::vertex::Vertex;



pub(crate) const NUM_VERTEX_CONFIGURATIONS: usize = 6;   //UPDATE THIS IF YOU ADD MORE VERTEX CONFIGURATIONS!

#[derive(Debug, Clone, Copy)]
pub(crate) enum VertexConfigration {
    SQUARE_SMALL_1,
    NEARLY_SQUARE_RECTANGLE_0,
        //update the NUM_VERTEX_CONFIGURATIONS constant if you add more vertex configurations! These must be in the same order as the struct in init.rs
    LINE_HORIZONTAL,
    LINE_VERTICAL,
    SMALL_ENTITY_WORKER,
    BIG_BASE_BASE_4X2,

} 

pub trait VertexConfigrationTrait {

    fn get_vertices(&self) -> [Vertex; 4];
}

//dynamically sized objects need to implement their own .get_vertices() method
impl VertexConfigrationTrait for VertexConfigration {
    #[inline(always)]
    fn get_vertices(&self) -> [Vertex; 4] {
        match self {
            VertexConfigration::SQUARE_SMALL_1 => SQUARE_1_1_VERTEX_CONF(),
            VertexConfigration::LINE_HORIZONTAL => SMALL_LINE_HORIZONTAL_VERTEX_CONF(),
            VertexConfigration::LINE_VERTICAL => SMALL_LINE_VERTICAL_VERTEX_CONF(),
            VertexConfigration::NEARLY_SQUARE_RECTANGLE_0 => SQUARE_0_0_VERTEX_CONF(),
            VertexConfigration::SMALL_ENTITY_WORKER => SMALL_ENTITY_WORKER(),
            VertexConfigration::BIG_BASE_BASE_4X2 => BIG_BASE_BASE_5X3() ,
            _ => todo!(),
        }
    }
}

#[allow(non_snake_case)]
#[inline(always)]
fn BIG_BASE_BASE_5X3()->  [Vertex; 4]{
    centered_rectangle(4.0, 2.0)
}

#[allow(non_snake_case)]      
#[inline(always)]
fn SMALL_ENTITY_WORKER()->  [Vertex; 4]{
   centered_rectangle(0.5, 0.5)
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




