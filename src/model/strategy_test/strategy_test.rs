use crate::game_objects::game_object::VisitableStructure;

use super::map_chunk::{MapChunk, ChunkInfo};


//the idea is to hold information about the current level, this includes the chunks that are currently loaded, the structures that are currently loaded and the borders of the level. This is supposed to be used for stuff like pathfinding and collision detection
pub struct LevelInfo{
    blocking_chunks: Vec<Box<dyn MapChunk>>,
    non_blocking_chunks: Vec<Box<dyn MapChunk>>,
    structures: Vec<Box<dyn VisitableStructure>>,
    borders: ChunkInfo,
}