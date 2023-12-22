
use crate::{game_objects::{buildings::start_obj::StartObj, game_object::DrawableObject}, rendering::sprites::{sprite_mapping::Sprite, vertex_configration::VertexConfigration}, controller::position::Position};

#[derive(Debug)]
pub(crate) struct Worker{
    home: Option< std::sync::Weak<StartObj>>,
    position: Position,
    id: u64,
    goal: Option<(f32, f32)>,
    speed: f32,
    next_tile: Option<(f32, f32)>,
    texture: Sprite,
}

impl Worker{
    pub(crate) fn new(home: Option<std::sync::Weak<StartObj>>, position: Position, id: u64, next_tile: (f32, f32)) -> Self{
        Self{
            home,
            position,
            id,
            goal: None,
            speed: 1.0f32,
            next_tile: None,
            texture : Sprite::WorkerBasic,
        }
    }

}

impl DrawableObject for Worker{
    fn get_position(&self) -> Position {
        self.position
    }

    fn get_x_y_values(&self) -> (f32, f32) {
        println!("Worker position: {:?}", self.position);
        (self.position.x, self.position.y)
    }

    fn get_size(&self) -> f32 {
        return 0.0;
    }

    fn get_texture(&self) -> &Sprite {
        return &self.texture;
    }

    fn process_animation(&mut self, delta_time: f64) {
        
    }

    fn get_vertex_configuration(&self) -> &VertexConfigration {
        &VertexConfigration::SMALL_ENTITY_WORKER
    }

    fn get_id(&self) -> u64 {
        return self.id;
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }
}