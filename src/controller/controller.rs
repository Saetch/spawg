

use std::sync::{Arc, atomic::AtomicBool};

use flume::Receiver;
use piston_window::MouseButton;
use winit::event::VirtualKeyCode;

use crate::controller::input::MouseInputType;

use super::input::ControllerInput;

use super::position::Position;



pub(crate) struct Controller{
    receiver: Receiver<ControllerInput>,
    cam_position:  
}




impl Controller{
    pub(crate) fn new( receiver: Receiver<ControllerInput>) -> Self{
        Self{
            receiver: receiver,
            cam_position: CamPostion { x: (0.0), y: (0.0) }
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
                ControllerInput::MouseInput { action } =>  self.mouse(action),
                ControllerInput::KeyboardInput { key, state } =>  self.keyboard(key),
                ControllerInput::WindowResized { dimensions } =>    println!("Got window resized!"),
            }
        }
    }


    pub(crate) fn handle_mouse_input(&mut self, buttons: (MouseButton, MouseButton), delta: MouseScrollDelta) {
        match MouseButton {
            (MouseButton::Left, MouseButton::Right) => {
                // Code für den Fall, wenn linke und rechte Maustaste gleichzeitig gedrückt werden
                match delta {
                }
            }
            (MouseButton::Left, _) => {
                // Code nur für linke Maustaste
                
            }
            (_, MouseButton::Right) => {
                // Code nur für rechte Maustaste
                
            }
            _ => {
                
            }
        }
    }


    pub(crate) fn keyboard(&mut self, input: Option<VirtualKeyCode>) {
        if let Some(key) = input {
            println!("Got keyboard input!");
            match key {
                VirtualKeyCode::Up => {
                    // Verarbeitung für Pfeiltaste nach oben
                    self.cam_position.y += 100.0;
                }
                VirtualKeyCode::Down => {
                    // Verarbeitung für Pfeiltaste nach unten
                    self.cam_position.y -= 100.0;
                }
                VirtualKeyCode::Left => {
                    // Verarbeitung für Pfeiltaste nach links
                    self.cam_position.x -= 100.0;
                }
                VirtualKeyCode::Right => {
                    // Verarbeitung für Pfeiltaste nach rechts
                    self.cam_position.x += 100.0;
                }
                _ => {}
            }
        }
    }







}