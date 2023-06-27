use std::sync::Arc;

use async_std::sync::RwLock;
use futures::join;

use crate::{game_objects::{buildings::debug_house::DebugHouse, debug::line::Line}, rendering::sprites::{sprite_mapping::Sprite, vertex_configration::VertexConfigration}, controller::position::Position};

use super::{model::Model, maze::maze::Maze};

pub(crate) enum Level{
    Initial,
    Maze,
}

impl Model{

    pub(crate) async fn load_level(&mut self, level: Level){

        self.reset_counters();

        match level{
            Level::Initial => self.load_initial_level().await,
            Level::Maze => self.load_maze_level().await,
        } 
    }

    pub(crate) async fn load_maze_level(&mut self){

        self.clear_logic_objects();
        self.clear_game_objects().await;
        self.clear_static_objects().await;
       
        
        //create a Maze
        let (maze, to_add_objects ) = Maze::new(23, 41, (-10.0, -5.0));
        let background_square = DebugHouse::new(Sprite::DarkBlue, Position { x: 1241.2, y: 1231.1 }, VertexConfigration::NEARLY_SQUARE_RECTANGLE_0);
        self.add_game_object(Arc::new(RwLock::new(background_square))).await;
        self.add_logic_object(Box::new(maze));
        self.add_game_objects(to_add_objects).await;


    }


    pub(crate) async fn load_initial_level(&mut self){
        
    
    }
}