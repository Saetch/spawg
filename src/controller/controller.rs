

use std::sync::RwLock;
use std::sync::{Arc, atomic::AtomicBool};

use flume::Receiver;
use piston_window::MouseButton;
use winit::event::VirtualKeyCode;

use crate::controller::input::MouseInputType; //

use super::input::ControllerInput;

use super::position::Position;





pub(crate) struct Controller{
    receiver: Receiver<ControllerInput>,
    position: Position
      
}




impl Controller{
    pub(crate) fn new( receiver: Receiver<ControllerInput>) -> Self{
        Self{
            receiver: receiver,
            position: Position { x: Arc::new(RwLock::new(0.0)), y: Arc::new(RwLock::new(0.0)) }
        }
    }


    pub(crate) fn run(&mut self){

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
                    let mut lock = self.position.y.write().unwrap();
                    *lock += 10.0;
                }
                VirtualKeyCode::Down => {
                    let mut lock = self.position.y.write().unwrap();
                    // Verarbeitung für Pfeiltaste nach unten
                    *lock -= 10.0;
                }
                VirtualKeyCode::Left => {
                    // Verarbeitung für Pfeiltaste nach links
                    let mut lock = self.position.x.write().unwrap();
                    *lock -= 10.0;
                }
                VirtualKeyCode::Right => {
                    // Verarbeitung für Pfeiltaste nach rechts
                    let mut lock = self.position.x.write().unwrap();
                    *lock += 10.0;
                }
                _ => {}
            }
        }
        print!("{:?}", self.position);
    }







}