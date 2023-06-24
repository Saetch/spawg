#![allow(dead_code, non_camel_case_types)]

use crate::rendering::vertex::Vertex;
#[derive(Debug, Clone, Copy)]
pub(crate) enum VertexConfigration {
    SQUARE_SMALL_1,
    SOMETHINGSOMETHING,
    ELSE
} 

pub trait VertexConfigrationTrait {

    fn get_vertices(&self, cam_size:(u32, u32)) -> [Vertex; 4];
}

impl VertexConfigrationTrait for VertexConfigration {
    #[inline(always)]
    fn get_vertices(&self, cam_size:(u32, u32)) -> [Vertex; 4] {
        match self {
            VertexConfigration::SQUARE_SMALL_1 => SQUARE_1_1_VERTEX_CONF(cam_size.0, cam_size.1),
            VertexConfigration::SOMETHINGSOMETHING => todo!(),
            VertexConfigration::ELSE => todo!(),
        }
    }
}

#[allow(non_snake_case)]      
#[inline(always)]
fn SQUARE_1_1_VERTEX_CONF(width: u32, height: u32)->  [Vertex; 4]{
   [
    Vertex { position: to_screen_coordinates([3.0, -3.0], width, height), tex_coords: [1.0, 1.0]}, // A
    Vertex { position: to_screen_coordinates([3.0, 3.0], width, height), tex_coords: [1.0, 0.0]}, // B
    Vertex { position: to_screen_coordinates([-3.0, 3.0], width, height), tex_coords: [0.0, 0.0] }, // C
    Vertex { position: to_screen_coordinates([-3.0, -3.0], width, height), tex_coords: [0.0, 1.0] }, // D
   ]
}


#[inline(always)]
fn to_screen_coordinates(mut position: [f32;2], width: u32, height: u32) -> [f32;2]{
    position[0] =  position[0] / (width as f32 / 2.0) ;
    position[1] =  position[1] / (height as f32 / 2.0) ;
    position
}

