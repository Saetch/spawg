use std::{sync::{atomic::{AtomicBool, Ordering}}, os::windows::thread, time::Duration};

use crate::game_objects::{game_object::DrawableObject, static_object::StaticObject};
use async_std::sync::{Arc, RwLock as AsyncRwLock};

//these types are just shorthand for the long type names, making it more easy to assess them
pub(crate) type GameObjectList = Arc<AsyncRwLock<Vec<Box<dyn DrawableObject + Send + Sync>>>>;
pub(crate) type StaticObjectList = Arc<AsyncRwLock<Vec<StaticObject>>>;


pub(crate) struct Model{
    pub(crate) running: Arc<AtomicBool>,  //<-- this is used to indicate whether the program should exit or not
    pub game_objects: GameObjectList,
    pub static_objects: StaticObjectList,
}



impl Model{
    pub(crate) fn new(running:Arc< AtomicBool>) -> Self{
        Self{
            running: running,
            game_objects: Arc::new(AsyncRwLock::new(Vec::new())),
            static_objects: Arc::new(AsyncRwLock::new(Vec::new())),
        }
    }


    


    pub(crate) fn run(&mut self){


        while self.running.load(Ordering::Relaxed){
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
}
