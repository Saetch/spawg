use crate::model::load_level_functions::Level;

use super::position::Position;

pub(crate) enum ControllerCommand{
    SpawnHouseAtPosition{         spawn_position: (f32, f32) },
    SpawnHouseAtPositionPixelated{spawn_position: (f32, f32) },



    LoadLevel(Level),
}