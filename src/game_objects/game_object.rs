use std::{fmt::Debug, time::Duration};

use crate::{rendering::{vertex::Vertex, sprites::{sprite_mapping::Sprite, vertex_configration::VertexConfigration}}, controller::position::Position};


///!!!This is the trait that all drawable objects have to implement, the implementations here are defaults and should be overridden if necessary !!!
pub(crate) trait DrawableObject: Debug + Send + Sync {
    

    fn get_position(&self) -> Position;
    fn get_x_y_values(&self) -> (f32, f32);
    fn get_size(&self) -> f32;
    fn get_texture(&self) -> &Sprite;



    //Consider making this interior mutable, in order to speed up access to these 
    fn process_animation(&mut self, delta_time: f64);
    fn process_logic(&mut self, delta_time: f64);
    fn get_vertex_configuration(&self) -> &VertexConfigration;
}


pub(crate) trait LogicObject: Debug{
    fn process_logic(&mut self, delta_time: Duration) -> bool;
}