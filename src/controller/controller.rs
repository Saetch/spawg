use std::sync::{Arc};
use async_std::sync::RwLock;
use flume::{Receiver, Sender};
use winit::event::VirtualKeyCode;
use crate::controller::input::MouseInputType;
use crate::model::load_level_functions::Level;
use super::controller_commands::ControllerCommand;
use super::input::ControllerInput;
use super::position::Position;
use super::renderer_commands::RendererCommand;


const CAM_INITIAL_WIDTH: f32 = 24.0;
const CAM_INITIAL_HEIGHT: f32 = CAM_INITIAL_WIDTH / CAM_RATIO;
const CAM_RATIO: f32 = 1280.0 / 720.0; //this is the ratio of the camera, it is used to calculate objects' positions on the screen


pub type SharablePosition = Arc<RwLock<Position>>;


pub(crate) struct Controller{
    receiver: Receiver<ControllerInput>,
    pub(crate) cam_position: SharablePosition,
    pub(crate) cam_proportions: Arc<RwLock<(f32, f32)>>,
    pub(crate) cam_directions: Arc<RwLock<(Direction, Direction)>>,
    
    
    model_sender: Sender<ControllerCommand>,//<-- this is used to send messages to the model, the model is supposed to evaluate them and process accordingly
                                             //for example: self.model_sender.send(ControllerCommand::SpawnHouseAtPosition { spawn_position: (0.0, 0.0) }).unwrap();
    renderer_sender: Sender<RendererCommand>,
}                                   
             
pub(crate) enum Direction{
    Positive,
    Negative,
    None,
}



impl Controller{
    pub(crate) fn new( receiver: Receiver<ControllerInput>, controller_to_model_sender: Sender<ControllerCommand>, renderer_sender: Sender<RendererCommand>) -> Self{
        Self{
            receiver: receiver,
            cam_position: Arc::new(RwLock::new(Position::new(0.0, 0.0))),
            model_sender: controller_to_model_sender,
            cam_proportions: Arc::new(RwLock::new((CAM_INITIAL_WIDTH, CAM_INITIAL_HEIGHT))),
            renderer_sender,
            cam_directions: Arc::new(RwLock::new((Direction::None, Direction::None))),
        }
    }


    pub(crate) async fn run(&mut self){
        self.model_sender.send(ControllerCommand::LoadLevel(Level::Maze)).unwrap();   //here we define what level should start. We use the controller for that, just because it is supposed to later decide what level to load anyways


        let mut personal_running_bool = true;       //we dont need a shared value, as we get notified of a shutdown via the channel
        while personal_running_bool{
            let received = self.receiver.recv().unwrap();
            match received{
                ControllerInput::Exit => {
                    personal_running_bool = false;
                    self.model_sender.send(ControllerCommand::Shutdown).unwrap();
                }
                ControllerInput::MouseInput { action } =>  self.handle_mouse_input(action),
                ControllerInput::KeyboardInput { key, state } =>  self.handle_keyboard_input(key).await,
                ControllerInput::WindowResized { dimensions } =>    println!("Got window resized!"),
            }
        }
    }


    pub(crate) fn handle_mouse_input(&mut self, buttons: MouseInputType) {
        println!("Got mouse input!");
        return;
        match buttons {
            MouseInputType::Move(x, y) => todo!(),
            MouseInputType::Click { button, state } => todo!(),
            MouseInputType::Scroll { delta, phase } => todo!(),
            MouseInputType::LeftWindow => todo!(),
            MouseInputType::EnteredWindow => todo!(),
        }
    }


    pub(crate) async fn handle_keyboard_input(&mut self, input: Option<VirtualKeyCode>) {
        if let Some(key) = input {
            println!("Got keyboard input!");
            match key {
                VirtualKeyCode::Up => {
                    // Verarbeitung für Pfeiltaste nach oben
                    let mut lock = self.cam_position.write().await;
                    (*lock).y += 0.1;
                }
                VirtualKeyCode::Down => {
                    let mut lock = self.cam_position.write().await;
                    // Verarbeitung für Pfeiltaste nach unten
                    (*lock).y -= 0.1;
                }
                VirtualKeyCode::Left => {
                    // Verarbeitung für Pfeiltaste nach links
                    let mut lock = self.cam_position.write().await;
                    (*lock).x -= 0.1;
                }
                VirtualKeyCode::Right => {
                    // Verarbeitung für Pfeiltaste nach rechts
                    let mut lock = self.cam_position.write().await;
                    (*lock).x += 0.1;
                }
                _ => {}
            }
        }
        print!("{:?}", self.cam_position);
    }







}