use std::time::Duration;

use crate::{game_objects::game_object::{VisitableStructure, LogicObject}, model::results::LogicResult};

use super::map_chunk::{MapChunk, ChunkInfo};


//the idea is to hold information about the current level, this includes the chunks that are currently loaded, the structures that are currently loaded and the borders of the level. This is supposed to be used for stuff like pathfinding and collision detection
#[derive(Debug)]
pub struct StratLevel{
    blocking_chunks: Vec<Box<dyn MapChunk>>,
    non_blocking_chunks: Vec<Box<dyn MapChunk>>,
    structures: Vec<Box<dyn VisitableStructure>>,
    borders: ChunkInfo,
    id: u32,
}

impl LogicObject for StratLevel{
    fn process_logic(&mut self, delta_time: Duration) -> LogicResult {
        LogicResult::None
    }

    fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    fn get_id(&self) -> u32 {
        self.id
    }
    
}