use std::ops::Index;
use std::sync::{Arc};
use async_std::sync::RwLock;
use flume::{Receiver, Sender};
use winit::event::{VirtualKeyCode, ElementState};
use winit::window::Window;
use crate::controller::input::MouseInputType;
use crate::model::load_level_functions::Level;
use super::controller_commands::ControllerCommand;
use super::input::ControllerInput;
use super::position::Position;
use super::renderer_commands::RendererCommand;


pub(crate) const CAM_INITIAL_WIDTH: f32 = 24.0;
pub(crate) const CAM_INITIAL_HEIGHT: f32 = CAM_INITIAL_WIDTH / CAM_RATIO;
const CAM_RATIO: f32 = 1280.0 / 720.0; //this is the ratio of the camera, it is used to calculate objects' positions on the screen


pub type SharablePosition = Arc<RwLock<Position>>;


pub(crate) struct Controller{
    receiver: Receiver<ControllerInput>,
    pub(crate) cam_position: SharablePosition,
    pub(crate) cam_proportions: Arc<RwLock<(f32, f32)>>,
    pub(crate) cam_directions: Arc<RwLock<(Direction, Direction)>>,
    personal_running_bool:  bool,
    
    model_sender: Sender<ControllerCommand>,//<-- this is used to send messages to the model, the model is supposed to evaluate them and process accordingly
                                             //for example: self.model_sender.send(ControllerCommand::SpawnHouseAtPosition { spawn_position: (0.0, 0.0) }).unwrap();
    renderer_sender: Sender<RendererCommand>,
    modifiers: Modifiers,
}

pub(crate) enum Modifier{
    Shift,
    Ctrl,
    Alt,
    RAlt,
}

struct Modifiers{
    modifiers: [bool; 4]
}



impl Modifiers{
    fn new() -> Self{
        Self{
            modifiers: [false; 4]
        }
    }

    fn set_modifier(&mut self, modifier: Modifier, state: bool){
        self.modifiers[modifier as usize] = state;
    }

    fn get_modifier(&self, modifier: Modifier) -> bool{
        self.modifiers[modifier as usize]
    }
}

#[derive(PartialEq)]
pub(crate) enum Direction{
    None,
    Positive,
    Negative,
    Muted,  //<- this means that both directional keys are pressed, so the camera should not move, but return moving once one of the keys is released
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
            modifiers: Modifiers::new(),
            personal_running_bool: true,
        }
    }


    pub(crate) async fn run(&mut self){
        self.model_sender.send(ControllerCommand::LoadLevel(Level::Maze)).unwrap();   //here we define what level should start. We use the controller for that, just because it is supposed to later decide what level to load anyways


        while self.personal_running_bool{
            let received = self.receiver.recv().unwrap();
            match received{
                ControllerInput::Exit => {
                    self.personal_running_bool = false;
                    self.model_sender.send(ControllerCommand::Shutdown).unwrap();
                }
                ControllerInput::MouseInput { action } =>  self.handle_mouse_input(action),
                ControllerInput::KeyboardInput { key, state } =>  self.handle_keyboard_input(key, state).await,
                ControllerInput::WindowResized { dimensions } =>    println!("Got window resized!"),
            }
        }
    }


    pub(crate) fn handle_mouse_input(&mut self, buttons: MouseInputType) {
        return;
        match buttons {
            MouseInputType::Move(x, y) => todo!(),
            MouseInputType::Click { button, state } => todo!(),
            MouseInputType::Scroll { delta, phase } => todo!(),
            MouseInputType::LeftWindow => todo!(),
            MouseInputType::EnteredWindow => todo!(),
        }
    }


    pub(crate) async fn handle_keyboard_input(&mut self, input: Option<VirtualKeyCode>, state: ElementState) {
        if let Some(key) = input {
            match key {
                VirtualKeyCode::Up => {
                    // Verarbeitung für Pfeiltaste nach oben
                    let mut lock = self.cam_directions.write().await;
                    (*lock).1 = match state {
                        ElementState::Pressed => match lock.1 {
                            Direction::None => Direction::Positive,
                            Direction::Negative => Direction::Muted,
                            Direction::Positive => Direction::Positive,
                            Direction::Muted => Direction::Muted,
                        },
                        ElementState::Released => match lock.1 {
                            Direction::None => Direction::None,
                            Direction::Negative => Direction::Negative,
                            Direction::Positive => Direction::None,
                            Direction::Muted => Direction::Negative,
                        },
                    };
                }
                VirtualKeyCode::Down => {
                    // Verarbeitung für Pfeiltaste nach unten
                    let mut lock = self.cam_directions.write().await;
                    (*lock).1 = match state {
                        ElementState::Pressed => match lock.1 {
                            Direction::None => Direction::Negative,
                            Direction::Negative => Direction::Negative,
                            Direction::Positive => Direction::Muted,
                            Direction::Muted => Direction::Muted,
                        },
                        ElementState::Released => match lock.1 {
                            Direction::None => Direction::None,
                            Direction::Negative => Direction::None,
                            Direction::Positive => Direction::Positive,
                            Direction::Muted => Direction::Positive,
                        },
                    };
                }
                VirtualKeyCode::Left => {
                    // Verarbeitung für Pfeiltaste nach links
                    let mut lock = self.cam_directions.write().await;
                    (*lock).0 = match state {
                        ElementState::Pressed => match lock.0{
                            Direction::None => Direction::Negative,
                            Direction::Negative => Direction::Negative,
                            Direction::Positive => Direction::Muted,
                            Direction::Muted => Direction::Muted,
                        },
                        ElementState::Released => match lock.0 {
                            Direction::None => Direction::None,
                            Direction::Negative => Direction::None,
                            Direction::Positive => Direction::Positive,
                            Direction::Muted => Direction::Positive,
                        },

                    };
                }
                VirtualKeyCode::Right => {
                    // Verarbeitung für Pfeiltaste nach rechts
                    let mut lock = self.cam_directions.write().await;
                    (*lock).0 = match state {
                        ElementState::Pressed => match lock.0 {
                            Direction::None => Direction::Positive,
                            Direction::Negative => Direction::Muted,
                            Direction::Positive => Direction::Positive,
                            Direction::Muted => Direction::Muted,
                        },
                        ElementState::Released => match lock.0 {
                            Direction::None => Direction::None,
                            Direction::Negative => Direction::Negative,
                            Direction::Positive => Direction::None,
                            Direction::Muted => Direction::Negative,
                        },
                    };
                }
                VirtualKeyCode::Escape => {
                    // Verarbeitung für Escape-Taste
                    self.model_sender.send(ControllerCommand::Shutdown).unwrap();
                    self.renderer_sender.send(RendererCommand::SHUTDOWN).unwrap();
                }
                VirtualKeyCode::Return => {
                    // Verarbeitung für Enter-Taste
                    match state {
                        ElementState::Pressed => {
                            if self.modifiers.get_modifier(Modifier::Alt) || self.modifiers.get_modifier(Modifier::RAlt) {
                                self.renderer_sender.send(RendererCommand::TOGGLE_FULLSCREEN).unwrap();
                            }
                        },
                        _ => ()
                    }                 
                }
                VirtualKeyCode::LAlt => {
                    // Verarbeitung für Alt-Taste
                    self.modifiers.set_modifier(Modifier::Alt, state == ElementState::Pressed);
                }
                VirtualKeyCode::LControl => {
                    // Verarbeitung für Strg-Taste
                    self.modifiers.set_modifier(Modifier::Ctrl, state == ElementState::Pressed);
                }
                VirtualKeyCode::LShift => {
                    // Verarbeitung für Shift-Taste
                    self.modifiers.set_modifier(Modifier::Shift, state == ElementState::Pressed);
                }
                VirtualKeyCode::RAlt => {
                    // Verarbeitung für Alt-Taste
                    self.modifiers.set_modifier(Modifier::RAlt, state == ElementState::Pressed);
                }

                _ => {}
            }
        }
    }







}