use std::sync::Arc;

use async_std::sync::RwLock;

use crate::game_objects::game_object::{DrawableObject, LogicObject};


pub(crate) type GameObjects = Vec<Arc<RwLock<dyn DrawableObject + Send + Sync>>>;
pub(crate) type LogicObjects = Vec<Arc<RwLock<dyn LogicObject + Send + Sync>>>;


pub(crate) enum LogicResult{
    CreateGameObjects{game_objects: GameObjects},
    CeateLogicObjects{logic_objects: LogicObjects},
    CreateGameAndLogicObjects{game_objects: GameObjects, logic_objects: LogicObjects},
    DestroyGameObjects{game_objects: GameObjects},
    DestroyLogicObjects{logic_objects: LogicObjects},
    DestroyGameAndLogicObjects{game_objects: GameObjects, logic_objects: LogicObjects},
    SpawnGameObjectTimer{spawn_timer: f64, function: fn(GameObjects) -> LogicResult},
    SpawnLogicObjectTimer{spawn_timer: f64, function: fn(LogicObjects) -> LogicResult},
    None,
    SelfDestruct
}