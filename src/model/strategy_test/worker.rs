
use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{game_objects::{buildings::start_obj::StartObj, game_object::{DrawableObject, VisitableStructure}}, rendering::sprites::{sprite_mapping::Sprite, vertex_configration::VertexConfigration}, controller::position::Position, model::results::LogicResult};

use super::strategy_test::StrategyLogicObject;

#[derive(Debug)]
pub(crate) struct Worker{
    home: Option< Arc<RwLock<StartObj>>>,
    position: Position,
    id: u64,
    origin_positiom: Position,
    goal: Option<(f32, f32)>,
    speed: f32,
    next_tile: Option<(f32, f32)>,
    path: Vec<(f32, f32)>,
    texture: Sprite,
    state: WorkerState,
}

#[derive(Debug)]
pub(crate) enum WorkerState{
    Idle,
    Moving,
    Working,
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
            origin_positiom: position,
            goal: None,
            speed: 0.5f32,
            next_tile: None,
            path: Vec::new(),
            texture : Sprite::WorkerBasic,
            state: WorkerState::Idle,
        }
    }

}

impl StrategyLogicObject for Worker{
    fn process_logic(&mut self, delta_time: std::time::Duration, blockers: &mut Vec<Box<dyn super::map_chunk::MapChunk>>, structures: &mut Vec<Arc<RwLock<dyn VisitableStructure>>>) -> LogicResult {
        let mut covered_distance = self.speed * delta_time.as_secs_f32();
        if self.path.len() == 0{
            return LogicResult::None;
        }
        let dist =self.position.distance(&self.path[0]);
        if  dist >= covered_distance{
            let direction = self.position.direction_to(&self.path[0]);
            self.position.x += direction.0 * covered_distance;
            self.position.y += direction.1 * covered_distance;
        }
        else{
            covered_distance -= dist;
            self.position = Position::new(self.path[0].0, self.path[0].1);
            self.path.remove(0);
            if self.path.len() == 0{
                self.state = WorkerState::Idle;
                println!("I am a worker and I am done! My id is: {} and I came from {:?}", self.id, self.origin_positiom);
            }
            else{
                let direction = self.position.direction_to(&self.path[0]);
                self.position.x += direction.0 * covered_distance;
                self.position.y += direction.1 * covered_distance;
            }
        }
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

    fn initialize_behavior(&mut self, blockers: &Vec<Box<dyn super::map_chunk::MapChunk>>, structures: &Vec<Arc<RwLock<dyn VisitableStructure>>>) {
        self.goal = Some(self.home.as_ref().unwrap().blocking_read().get_entry_point().get_x_y_values());
        let start_position = &self.position;
        self.path = if let Some(path) = start_position.find_path_to(self.goal.as_ref().unwrap(), blockers, structures){ path } else{
            Vec::new()
        };
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