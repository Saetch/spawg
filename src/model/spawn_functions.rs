use std::sync::Arc;

use async_std::sync::RwLock;

use crate::{game_objects::{buildings::debug_house::DebugHouse, game_object::DrawableObject}, rendering::sprites::{sprite_mapping::Sprite, vertex_configration::VertexConfigration}, controller::position::Position};

use super::model::Model;

impl Model{

    pub(crate) async fn spawn_house_at_position(&mut self, pos: (f32, f32)){
        let house = DebugHouse::new(Sprite::DwarfBaseHouse, Position::new(pos.0, pos.1), VertexConfigration::SQUARE_SMALL_1);
        self.add_game_object(Arc::new(RwLock::new(house)));
    }
}