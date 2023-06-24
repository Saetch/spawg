use std::{sync::{atomic::{AtomicBool, Ordering}}, os::windows::thread, time::Duration};

use crate::{game_objects::{game_object::DrawableObject, static_object::StaticObject, buildings::debug_house::DebugHouse}, controller::{position::Position, controller_commands::ControllerCommand}, rendering::sprites::{sprite_mapping::Sprite, vertex_configration::VertexConfigration}};
use async_std::{sync::{Arc, RwLock as AsyncRwLock}};
use flume::Receiver;

//these types are just shorthand for the long type names, making it more easy to assess them
pub(crate) type GameObjectList = Arc<AsyncRwLock<Vec<Box<dyn DrawableObject + Send + Sync>>>>;
pub(crate) type StaticObjectList = Arc<AsyncRwLock<Vec<StaticObject>>>;


pub(crate) struct Model{
    pub(crate) running: Arc<AtomicBool>,  //<-- this is used to indicate whether the program should exit or not
    pub game_objects: GameObjectList,
    pub static_objects: StaticObjectList,
    controller_receiver: Receiver<ControllerCommand>
}



impl Model{
    pub(crate) fn new(running:Arc< AtomicBool>, controller_to_model_receiver: Receiver<ControllerCommand>) -> Self{
        Self{
            running: running,
            game_objects: Arc::new(AsyncRwLock::new(Vec::new())),
            static_objects: Arc::new(AsyncRwLock::new(Vec::new())),
            controller_receiver: controller_to_model_receiver
        }
    }





    pub(crate) async fn run(&mut self){


        while self.running.load(Ordering::Relaxed){

            while let Ok(command) = self.controller_receiver.try_recv(){
                self.process_controller_command(command).await;
            }

            //do stuff
            self.compute_logic_objects();
            //sleep 1 second
            std::thread::sleep(Duration::from_secs(1));
        }   



        println!("Model thread exited");
    }




    #[inline(always)]
    fn compute_logic_objects(&mut self){
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