
use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{game_objects::{buildings::start_obj::StartObj, game_object::DrawableObject}, rendering::sprites::{sprite_mapping::Sprite, vertex_configration::VertexConfigration}, controller::position::Position, model::results::LogicResult};

use super::strategy_test::StrategyLogicObject;

#[derive(Debug)]
pub(crate) struct Worker{
    home: Option< Arc<RwLock<StartObj>>>,
    position: Position,
    id: u64,
    goal: Option<(f32, f32)>,
    speed: f32,
    next_tile: Option<(f32, f32)>,
    texture: Sprite,
}

impl Worker{
    pub(crate) fn new(home: Option<Arc<RwLock<StartObj>>>, position: Position, id: u64) -> Self{
        if let Some(home_base) = &home{
            home_base.blocking_write().add_associated_unit(id);
        }
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

impl StrategyLogicObject for Worker{
    fn process_logic(&mut self, delta_time: std::time::Duration, blockers: &mut Vec<Box<dyn super::map_chunk::MapChunk>>, structures: &mut Vec<Box<dyn crate::game_objects::game_object::VisitableStructure>>) -> LogicResult {
        LogicResult::None
    }

    fn set_id(&mut self, id: u64) {
        let old_id = self.id;
        self.id = id;

        if old_id != self.id{
            if let Some(home_base) = &self.home{
                let mut lock = home_base.blocking_write();
                lock.remove_associated_unit(old_id);
                lock.add_associated_unit(self.id);
            }
        }

    }

    fn get_id(&self) -> u64 {
        self.id
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