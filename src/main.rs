use std::{sync::{atomic::AtomicBool, Arc, RwLock}, thread, time::{Duration, SystemTime}};

use async_std::task::block_on;
use controller::{controller::Controller, position::Position, controller_commands::ControllerCommand};
use flume::Receiver;
use model::model::Model;

use crate::{rendering::wgpurenderer::Renderer, controller::controller::SharablePosition};


mod game_objects;
mod rendering;
mod model;
mod controller;

#[async_std::main]
pub async fn main() {

    env_logger::init();     //wgpu logs per default to the env_logger. If we don't initialize it, we only get very basic and not very helpful errors

    let (controller_sender, controller_receiver) = flume::unbounded();  //this channel is used to send messages from the event loop to the controller 
    let (controller_to_model_sender, controller_to_model_receiver) = flume::unbounded();  //this channel is used to send messages from the controller to the model
    let (controller_to_renderer_sender, controller_to_renderer_receiver) = flume::unbounded();  //this channel is used to send messages from the controller to the renderer
    
    let mut join_handles_vec = Vec::new();     //this vector will be used to store all the join handles of the threads that are spawned

    let running = AtomicBool::new(true);  //<-- this is used to indicate whether the program should exit or not
    let running = Arc::new(running);


    //spawn the model thread
    let mut model = Model::new(running.clone(), controller_to_model_receiver);
    println!("Time: {}", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis());
    let model_thread = thread::spawn(move || { 
        println!("Time: {}", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis());
    });
    println!("Time in first thread: {}", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis());


    let model_thread = thread::spawn(move || { 
        block_on(model.run());
    });
    join_handles_vec.push(model_thread);
    let mut controller = Controller::new(controller_receiver, controller_to_model_sender, controller_to_renderer_sender);
    let cam_pos: SharablePosition = controller.cam_position.clone();
    let controller_thread = thread::spawn(move || { 
        controller.run();
    });
    join_handles_vec.push(controller_thread);




    Renderer::run(running, join_handles_vec, controller_sender, controller_to_renderer_receiver, cam_pos).await;
}