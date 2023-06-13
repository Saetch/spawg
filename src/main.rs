use std::{sync::{atomic::AtomicBool, Arc}, thread};

use model::model::Model;

use crate::rendering::wgpurenderer::Renderer;


mod game_objects;
mod rendering;
mod model;


#[tokio::main]
pub async fn main() {

    env_logger::init();     //wgpu logs per default to the env_logger. If we don't initialize it, we only get very basic and not very helpful errors
    

    let mut join_handles_vec = Vec::new();     //this vector will be used to store all the join handles of the threads that are spawned

    let running = AtomicBool::new(true);  //<-- this is used to indicate whether the program should exit or not
    let running = Arc::new(running);


    //spawn the model thread
    let mut model = Model::new(running.clone());
    let model_thread = thread::spawn(move || { 
        model.run();
    });
    join_handles_vec.push(model_thread);




    Renderer::run(running, join_handles_vec).await;
}