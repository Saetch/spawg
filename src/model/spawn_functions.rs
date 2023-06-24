use flume::r#async;

use crate::{game_objects::buildings::debug_house::DebugHouse, rendering::sprites::{sprite_mapping::Sprite, vertex_configration::VertexConfigration}, controller::position::Position};

use super::model::Model;

impl Model{
    pub(crate) async fn spawn_house_at_position(&self, pos: (f32, f32)){
        let mut lock = self.game_objects.write().await;
        lock.push(Box::new(DebugHouse::new(Sprite::DwarfBaseHouse, Position::new(pos.0, pos.1), VertexConfigration::SQUARE_SMALL_1)));
    }
}