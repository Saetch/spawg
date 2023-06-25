use std::{sync::{atomic::{AtomicBool, Ordering}}, os::windows::thread, time::Duration, future::Future};

use crate::{game_objects::{game_object::DrawableObject, static_object::StaticObject, buildings::debug_house::DebugHouse}, controller::{position::Position, controller_commands::ControllerCommand}, rendering::sprites::{sprite_mapping::Sprite, vertex_configration::VertexConfigration}};
use async_std::{sync::{Arc, RwLock as AsyncRwLock}};
use flume::Receiver;

//these types are just shorthand for the long type names, making it more easy to assess them
pub(crate) type GameObjectList = Arc<AsyncRwLock<Vec<Arc<AsyncRwLock<dyn DrawableObject + Send + Sync >>>>>;
pub(crate) type StaticObjectList = Arc<AsyncRwLock<Vec<StaticObject>>>;
type GameState = Arc<AsyncRwLock<dyn GameStateTrait>>;

pub(crate) struct Model{
    pub(crate) running: Arc<AtomicBool>,  //<-- this is used to indicate whether the program should exit or not
    pub game_objects: GameObjectList,
    pub static_objects: StaticObjectList,
    controller_receiver: Receiver<ControllerCommand>,
    pub state: i32,
    pub logic_function: fn(&mut Self, delta_time: Duration),
    game_state: GameState
}


//the trait that all different game states should implement, the SEND + SYNC is needed to be able to send the game state between threads
pub(crate) trait GameStateTrait: Send + Sync{
    fn update(&mut self, delta_time: Duration);
}

struct MazeGameState{

}

impl GameStateTrait for MazeGameState{
    fn update(&mut self, delta_time: Duration){

    }
}



impl Model{
    pub(crate) fn new(running:Arc< AtomicBool>, controller_to_model_receiver: Receiver<ControllerCommand>) -> Self{
        Self{
            running: running,
            game_objects: Arc::new(AsyncRwLock::new(Vec::new())),
            static_objects: Arc::new(AsyncRwLock::new(Vec::new())),
            controller_receiver: controller_to_model_receiver,
            state: 0,
            logic_function: Self::empty,
            game_state: Arc::new(AsyncRwLock::new(MazeGameState{})),
        }
    }





    pub(crate) async fn run(&mut self){


        //for debug purposes, this can be removed, I just want to try the maze logic
        self.logic_function = Self::maze_logic;
        let mut last_execution_time = std::time::Instant::now();
        
        while self.running.load(Ordering::Relaxed){

            while let Ok(command) = self.controller_receiver.try_recv(){
                self.process_controller_command(command).await;
            }

            //do stuff
            (self.logic_function)(self, last_execution_time.elapsed());    //what this does is that it calls whatever function we have stored in logic_function. This means that we can easily change what the model is supposed to do.
            last_execution_time = std::time::Instant::now();


            //sleep 1 second
            std::thread::sleep(Duration::from_secs(1));
        }   



        println!("Model thread exited");
    }

     fn empty(&mut self, delta_time: Duration){
        println!("Model processing empty logic. There is nothing to do");
    }


    #[inline(always)]
     fn maze_logic(&mut self, delta_time: Duration){
        println!("Model thread maze logic. Time elapsed: {}", delta_time.as_secs_f32());
    }


    #[inline(always)]
    fn compute_logic_objects(&mut self, delta_time: Duration){
        println!("Model thread computing logic objects");
    }


    async fn process_controller_command(&mut self, command: ControllerCommand){
        match command{
            ControllerCommand::SpawnHouseAtPosition { spawn_position } => {
                     self.spawn_house_at_position(spawn_position).await;
                }
            ControllerCommand::SpawnHouseAtPositionPixelated { spawn_position: _ } => {},
            ControllerCommand::LoadLevel(level) => self.load_level(level).await,
         }
    }
    
}