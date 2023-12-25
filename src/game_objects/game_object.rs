use std::{fmt::Debug, time::Duration};

use crate::{rendering::{sprites::{sprite_mapping::Sprite, vertex_configration::VertexConfigration}}, controller::position::Position, model::{results::LogicResult, strategy_test::map_chunk::{MapChunk, ChunkInfo}}};


///!!!This is the trait that all drawable objects have to implement, the implementations here are defaults and should be overridden if necessary !!!
pub(crate) trait DrawableObject: Debug + Send + Sync {
    

    fn get_position(&self) -> Position;
    fn get_x_y_values(&self) -> (f32, f32);
    fn get_size(&self) -> f32;
    fn get_texture(&self) -> &Sprite;



    //Consider making this interior mutable, in order to speed up access to these 
    fn process_animation(&mut self, delta_time: f64);
    fn get_vertex_configuration(&self) -> &VertexConfigration;


    fn get_id(&self) -> u64;
    fn set_id(&mut self, id: u64);
}

pub (crate) trait VisitableStructure: Debug + Send + Sync {
    fn get_entry_point(&self) -> Position;
    fn get_blocking_chunk(&self) -> ChunkInfo;
}


pub(crate) trait LogicObject: Debug{
    fn process_logic(&mut self, delta_time: Duration) -> LogicResult;
    fn set_id(&mut self, id: u32);
    fn get_id(&self) -> u32;
}