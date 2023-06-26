use std::{sync::{atomic::{AtomicBool, Ordering}, RwLock}, os::windows::thread, time::Duration, future::Future, ops::Index};

use crate::{game_objects::{game_object::{DrawableObject, LogicObject}, static_object::StaticObject, buildings::debug_house::DebugHouse}, controller::{position::Position, controller_commands::ControllerCommand}, rendering::sprites::{sprite_mapping::Sprite, vertex_configration::VertexConfigration}};
use async_std::{sync::{Arc, RwLock as AsyncRwLock}, task::block_on};
use flume::{Receiver, r#async};

use super::results::LogicResult;

//these types are just shorthand for the long type names, making it more easy to assess them
pub(crate) type GameObjectList = Arc<AsyncRwLock<Vec<Arc<AsyncRwLock<dyn DrawableObject + Send + Sync >>>>>;
pub(crate) type StaticObjectList = Arc<AsyncRwLock<Vec<StaticObject>>>;
pub(crate) type LogicObjects = Vec<Box<dyn LogicObject>>;

pub(crate) struct Model{
    pub(crate) running: bool,  //<-- this is used to indicate whether the program should exit or not
    game_objects: GameObjectList,           //we need to make sure to increment the counters and thus we can't let the controller directly access these, instead we have to use the commands
    pub static_objects: StaticObjectList,
    pub(crate) logic_objects: LogicObjects,


    controller_receiver: Receiver<ControllerCommand>,
    pub state: i32,
    

    static_object_id_counter: u64,
    game_object_id_counter: u64,
    logic_object_id_counter: u32,
}


//this struct is to group all the actions that the model has to do after the logic has been processed in order to deal with newly created objects and objects that need to be destroyed

struct LogicActions{
    create_game_objects: Vec<Arc<AsyncRwLock<dyn DrawableObject + Send + Sync>>>,
    create_static_objects: Vec<StaticObject>,
    create_logic_objects: LogicObjects,
    destroy_game_objects: Vec<u64>,
    destroy_static_objects: Vec<u64>,
    destroy_logic_objects: Vec<u64>,
}




impl Model{
    pub(crate) fn new(controller_to_model_receiver: Receiver<ControllerCommand>, game_objects: GameObjectList) -> Self{
        Self{
            running: true,
            game_objects: game_objects,
            static_objects: Arc::new(AsyncRwLock::new(Vec::new())),
            controller_receiver: controller_to_model_receiver,
            state: 0,
            logic_objects: Vec::new(),
            static_object_id_counter: 0,
            game_object_id_counter: 0,
            logic_object_id_counter: 0,
        }
    }





    pub(crate) async fn run(&mut self){


        let mut loop_helper = spin_sleep::LoopHelper::builder()
        .report_interval_s(0.5) // report every half a second
        .build_with_target_rate(60.0);
        while self.running{

            while let Ok(command) = self.controller_receiver.try_recv(){
                self.process_controller_command(command).await;
            }

            //do stuff
            self.update(loop_helper.loop_start()).await;    //what this does is that it calls whatever function we have stored in logic_function. This means that we can easily change what the model is supposed to do.

            loop_helper.loop_sleep();

        }   



        println!("Model thread exited");
    }




    #[inline(always)]
     async fn update(&mut self, delta_time: Duration){
        //in the maze, only logic objects actually do something, so there is no need to call anything else than compute_logic_objects
        self.compute_logic_objects(delta_time).await;
    }



    #[inline(always)]
    async fn compute_logic_objects(&mut self, delta_time: Duration){
        let mut logic_actions = LogicActions::new();
        for id in 0..self.logic_objects.len(){
            let logic_object = self.logic_objects[id].as_mut();
            let logic_result = logic_object.process_logic(delta_time);
            process_logic_result(logic_result, &mut logic_actions);
        }
        if logic_actions.destroy_game_objects.len() > 0 {
            self.destroy_game_objects(logic_actions.destroy_game_objects).await;
        }
        if logic_actions.create_game_objects.len() > 0{
            self.add_game_objects(logic_actions.create_game_objects).await;
        }
        if logic_actions.create_logic_objects.len() > 0 {
            self.add_logic_objects(logic_actions.create_logic_objects);
        }
        if logic_actions.destroy_logic_objects.len() > 0{
            self.destroy_logic_objects(logic_actions.destroy_logic_objects);
        }
    }



    #[inline(always)]
    fn destroy_logic_objects(&mut self, logic_objects: Vec<u64>){

        let mut indices = Vec::with_capacity(logic_objects.len());

        for logic_object_id in logic_objects{
            for (index, logic_object) in self.logic_objects.iter().enumerate(){
                if logic_object.get_id() == logic_object_id as u32{
                    indices.push(index);
                    break;
                }
            }
        }

        while let Some(index) = indices.pop(){
            self.logic_objects.remove(index);
        }
    }

    #[inline(always)]
    async fn destroy_game_objects(&mut self, game_objects: Vec<u64>){
        let mut indices = Vec::with_capacity(game_objects.len());

        for game_object_id in game_objects{
            for (index, game_object) in self.game_objects.write().await.iter().enumerate(){
                if game_object.read().await.get_id() == game_object_id{
                    indices.push(index);
                    break;
                }
            }
        }

        
        while let Some(index) = indices.pop(){
            self.game_objects.write().await.remove(index);
        }
    }


    async fn process_controller_command(&mut self, command: ControllerCommand){
        match command{
            ControllerCommand::SpawnHouseAtPosition { spawn_position } => {
                     self.spawn_house_at_position(spawn_position).await;
                }
            ControllerCommand::SpawnHouseAtPositionPixelated { spawn_position: _ } => {},
            ControllerCommand::LoadLevel(level) => self.load_level(level).await,
            ControllerCommand::Shutdown => self.running = false,
         }
    }


    pub(super) fn reset_counters(&mut self){
        self.static_object_id_counter = 0;
        self.game_object_id_counter = 0;
        self.logic_object_id_counter = 0;
    }
    

    pub(super) async fn add_game_object(&mut self, game_object: Arc<AsyncRwLock<dyn DrawableObject + Send + Sync>>){
        game_object.write().await.set_id(self.game_object_id_counter);
        self.game_objects.write().await.push(game_object);
        self.game_object_id_counter += 1;
    }

    pub(super) async fn add_game_objects(&mut self, game_objects: Vec<Arc<AsyncRwLock<dyn DrawableObject + Send + Sync>>>){
        for game_object in &game_objects{
            game_object.write().await.set_id(self.game_object_id_counter);
            self.game_object_id_counter += 1;
        }
        self.game_objects.write().await.extend(game_objects);       
    }

    pub(super)  fn add_logic_object(&mut self, mut logic_object: Box<dyn LogicObject>){
        logic_object.set_id(self.logic_object_id_counter);
        self.logic_objects.push(logic_object);
        self.logic_object_id_counter += 1;
    }

    pub(super) fn add_logic_objects(&mut self, mut logic_objects: Vec<Box<dyn LogicObject>>){
        for logic_object in &mut logic_objects{
            logic_object.set_id(self.logic_object_id_counter);
            self.logic_object_id_counter += 1;
        }
        self.logic_objects.extend(logic_objects);
    }

    pub(super) async fn clear_game_objects(&mut self){
        self.game_objects.write().await.clear();
        self.game_object_id_counter = 0;
    }

    pub(super) fn clear_logic_objects(&mut self){
        self.logic_objects.clear();
        self.logic_object_id_counter = 0;
    }

    pub(super) async fn clear_static_objects(&mut self){
        self.static_objects.write().await.clear();
        self.static_object_id_counter = 0;
    }

}


#[inline(always)]
fn process_logic_result(actions: LogicResult, after_processing_management_actions:  &mut LogicActions)  {
    match actions {
        LogicResult::CreateGameObjects { game_objects } => { after_processing_management_actions.add_create_game_objects(game_objects);},
        LogicResult::CeateLogicObjects { logic_objects } => { after_processing_management_actions.add_create_logic_objects(logic_objects);},
        LogicResult::CreateGameAndLogicObjects { game_objects, logic_objects } => {after_processing_management_actions.add_create_game_objects(game_objects); after_processing_management_actions.add_create_logic_objects(logic_objects);},
        LogicResult::CreateAndDestroyGameObjects { game_objects_to_create, game_objects_to_destroy } => { after_processing_management_actions.add_create_game_objects(game_objects_to_create); after_processing_management_actions.add_destroy_game_objects(game_objects_to_destroy);},
        LogicResult::CreateAndDestroyLogicObjects { logic_objects_to_create, logic_objects_to_destroy } => { after_processing_management_actions.add_create_logic_objects(logic_objects_to_create); after_processing_management_actions.add_destroy_logic_objects(logic_objects_to_destroy);},
        LogicResult::DestroyGameObjects { game_objects } => { after_processing_management_actions.add_destroy_game_objects(game_objects);},
        LogicResult::DestroyLogicObjects { logic_objects } => { after_processing_management_actions.add_destroy_logic_objects(logic_objects);},
        LogicResult::DestroyGameAndLogicObjects { game_objects, logic_objects } => { after_processing_management_actions.add_destroy_game_objects(game_objects); after_processing_management_actions.add_destroy_logic_objects(logic_objects);},
        LogicResult::SpawnGameObjectTimer { spawn_timer, function } => todo!(),
        LogicResult::SpawnLogicObjectTimer { spawn_timer, function } => todo!(),
        LogicResult::None => (),
    }
}


impl LogicActions{
    fn new() -> LogicActions{
        LogicActions {
            create_game_objects: Vec::new(),
            create_static_objects: Vec::new(),
            create_logic_objects: Vec::new(),
            destroy_game_objects: Vec::new(),
            destroy_static_objects: Vec::new(),
            destroy_logic_objects: Vec::new(),
        }
    }


    fn add_create_game_objects(&mut self, game_object: Vec<Arc<AsyncRwLock<dyn DrawableObject + Send + Sync>>>){
        self.create_game_objects.extend(game_object);
    }

    fn add_create_static_objects(&mut self, static_object: Vec<Arc<AsyncRwLock<StaticObject>>>){
        todo!();
    }

    fn add_create_logic_objects(&mut self, logic_object: Vec<Box<dyn LogicObject>>){
        self.create_logic_objects.extend(logic_object);
    }

    fn add_destroy_game_objects(&mut self, game_object: Vec<u64>){
        self.destroy_game_objects.extend(game_object);
    }

    fn add_destroy_static_objects(&mut self, static_object: Vec<u64>){
        todo!();
    }

    fn add_destroy_logic_objects(&mut self, logic_object: Vec<u64>){
        self.destroy_logic_objects.extend(logic_object);
    }


}