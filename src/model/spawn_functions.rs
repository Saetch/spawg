use std::sync::Arc;

use async_std::sync::RwLock;

use crate::{game_objects::{buildings::debug_house::DebugHouse, game_object::DrawableObject}, rendering::sprites::{sprite_mapping::Sprite, vertex_configration::VertexConfigration}, controller::position::Position};

use super::model::Model;

impl Model{

    async fn add_object(&self, object: Arc<RwLock<dyn DrawableObject + Send + Sync>>){
        let mut lock = self.game_objects.write().await;
        lock.push(object);
    }

    pub(crate) async fn spawn_house_at_position(&self, pos: (f32, f32)){
        let mut lock = self.game_objects.write().await;
        lock.push(Arc::new(RwLock::new(DebugHouse::new(Sprite::DwarfBaseHouse, Position::new(pos.0, pos.1), VertexConfigration::SQUARE_SMALL_1))));
    }
}