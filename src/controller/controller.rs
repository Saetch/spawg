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
}