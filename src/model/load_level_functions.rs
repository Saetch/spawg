use crate::{game_objects::buildings::debug_house::DebugHouse, rendering::sprites::{sprite_mapping::Sprite, vertex_configration::VertexConfigration}, controller::position::Position};

use super::model::Model;

pub(crate) enum Level{
    Initial,
}

impl Model{

    pub(crate) async fn load_level(&mut self, level: Level){
        match level{
            Level::Initial => self.load_initial_level().await,
        } 
    }


    pub(crate) async fn load_initial_level(&mut self){
        let mut lock = self.game_objects.write().await;
        
        lock.clear();


        //add a debug house, here other stuff is loaded aswell
        lock.push(Box::new(DebugHouse::new(Sprite::DwarfBaseHouse, Position::new(0.0, 0.0), VertexConfigration::SQUARE_1_1)));

        drop(lock);         //this would be done automatically, but we drop it manually, so that it is released faster if we do anything else in this function after this point
    }
}