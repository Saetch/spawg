use std::sync::{Arc, atomic::AtomicBool};

use flume::Receiver;

use super::input::ControllerInput;

pub(crate) struct Controller{
    receiver: Receiver<ControllerInput>,
}


impl Controller{
    pub(crate) fn new( receiver: Receiver<ControllerInput>) -> Self{
        Self{
            receiver: receiver,
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
                ControllerInput::MouseInput { action } =>  println!("Got mouse input!"),
                ControllerInput::KeyboardInput { key, state } =>  println!("Got keyboard input!"),
                ControllerInput::WindowResized { dimensions } =>    println!("Got window resized!"),
            }
        }
    }
}