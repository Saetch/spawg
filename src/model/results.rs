use std::sync::Arc;

use async_std::sync::RwLock;

use crate::game_objects::game_object::{DrawableObject, LogicObject};

use super::model::LogicObjects;


pub(crate) type GameObjects = Vec<Arc<RwLock<dyn DrawableObject + Send + Sync>>>;


#[derive(Debug)]
pub(crate) enum LogicResult{
    CreateGameObjects{game_objects: GameObjects},
    CeateLogicObjects{logic_objects: LogicObjects},
    CreateGameAndLogicObjects{game_objects: GameObjects, logic_objects: LogicObjects},
    CreateAndDestroyGameObjects{game_objects_to_create: GameObjects, game_objects_to_destroy: Vec<u64>},
    CreateAndDestroyLogicObjects{logic_objects_to_create: LogicObjects, logic_objects_to_destroy: Vec<u64>},
    DestroyGameObjects{game_objects: Vec<u64>},
    DestroyLogicObjects{logic_objects: Vec<u64>},
    DestroyGameAndLogicObjects{game_objects: Vec<u64>, logic_objects: Vec<u64>},
    SpawnGameObjectTimer{spawn_timer: f64, function: fn(GameObjects) -> LogicResult},
    SpawnLogicObjectTimer{spawn_timer: f64, function: fn(LogicObjects) -> LogicResult},
    None,
    
}