use std::sync::RwLock;
use std::sync::{Arc};
use flume::{Receiver, Sender};
use winit::event::VirtualKeyCode;
use crate::controller::input::MouseInputType;
use crate::model::load_level_functions::Level;
use super::controller_commands::ControllerCommand;
use super::input::ControllerInput;
use super::position::Position;
use super::renderer_commands::RendererCommand;


const CAM_INITIAL_WIDTH: u32 = 24;
const CAM_INITIAL_HEIGHT: u32 = 14;
const CAM_RATIO: f32 = 1280.0 / 720.0; //this is the ratio of the camera, it is used to calculate objects' positions on the screen


pub type SharablePosition = Arc<RwLock<Position>>;


pub(crate) struct Controller{
    receiver: Receiver<ControllerInput>,
    pub(crate) cam_position: SharablePosition,
    pub(crate) cam_proportions: Arc<RwLock<(u32, u32)>>,
    model_sender: Sender<ControllerCommand>,//<-- this is used to send messages to the model, the model is supposed to evaluate them and process accordingly
                                             //for example: self.model_sender.send(ControllerCommand::SpawnHouseAtPosition { spawn_position: (0.0, 0.0) }).unwrap();
    renderer_sender: Sender<RendererCommand>,

}                                   
             




impl Controller{
    pub(crate) fn new( receiver: Receiver<ControllerInput>, controller_to_model_sender: Sender<ControllerCommand>, renderer_sender: Sender<RendererCommand>) -> Self{
        Self{
            receiver: receiver,
            cam_position: Arc::new(RwLock::new(Position::new(0.0, 0.0))),
            model_sender: controller_to_model_sender,
            cam_proportions: Arc::new(RwLock::new((CAM_INITIAL_WIDTH, CAM_INITIAL_HEIGHT))),
            renderer_sender
        }
    }


    pub(crate) fn run(&mut self){
        self.model_sender.send(ControllerCommand::LoadLevel(Level::Initial)).unwrap();   //here we define what level should start. We use the controller for that, just because it is supposed to later decide what level to load anyways


        let mut personal_running_bool = true;       //we dont need a shared value, as we get notified of a shutdown via the channel
        while personal_running_bool{
            let received = self.receiver.recv().unwrap();
            match received{
                ControllerInput::Exit => {
                    personal_running_bool = false;
                }
                ControllerInput::MouseInput { action } =>  self.handle_mouse_input(action),
                ControllerInput::KeyboardInput { key, state } =>  self.handle_keyboard_input(key),
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


    pub(crate) fn handle_keyboard_input(&mut self, input: Option<VirtualKeyCode>) {
        if let Some(key) = input {
            println!("Got keyboard input!");
            match key {
                VirtualKeyCode::Up => {
                    // Verarbeitung für Pfeiltaste nach oben
                    let mut lock = self.cam_position.write().unwrap();
                    (*lock).y += 10.0;
                }
                VirtualKeyCode::Down => {
                    let mut lock = self.cam_position.write().unwrap();
                    // Verarbeitung für Pfeiltaste nach unten
                    (*lock).y -= 10.0;
                }
                VirtualKeyCode::Left => {
                    // Verarbeitung für Pfeiltaste nach links
                    let mut lock = self.cam_position.write().unwrap();
                    (*lock).x -= 10.0;
                }
                VirtualKeyCode::Right => {
                    // Verarbeitung für Pfeiltaste nach rechts
                    let mut lock = self.cam_position.write().unwrap();
                    (*lock).x += 10.0;
                }
                _ => {}
            }
        }
        print!("{:?}", self.cam_position);
    }







}