use crate::{game_objects::game_object::{self, DrawableObject}, controller::position::Position, rendering::{sprites::{sprite_mapping::Sprite, vertex_configration::VertexConfigration}, vertex::Vertex}};
#[derive(Debug)]
pub(crate) struct DebugHouse{
    pub(crate) texture: Sprite,
    pub position: Position,
    pub vertices: VertexConfigration,

}


impl DebugHouse {
    pub(crate) fn new(texture: Sprite, position: Position, vertices: VertexConfigration) -> Self{
        Self{
            texture,
            position,
            vertices,
        }
    }
}

impl DrawableObject for DebugHouse{
    fn get_position(&self) -> Position {
        self.position
    }

    fn process_logic(&mut self, delta_time: f64) {
        
    }

    fn get_x_y_values(&self) -> (f32, f32) {
        (self.position.x, self.position.y)
    }

    fn get_size(&self) -> f32 {
        todo!()
    }

    fn get_texture(&self) -> &Sprite {
        &self.texture
    }

    fn process_animation(&mut self, delta_time: f64) {
    }

    fn get_vertex_configuration(&self) -> &VertexConfigration {
        &self.vertices
    }


}