use std::sync::{Arc, RwLock};
#[derive(Debug)]

pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Position {
            x: x,
            y: y,
        }
    }

    pub fn get_x(&self) -> f32 {
       self.x
    }

    pub fn set_x(&mut self, value: f32) {
        self.x = value;
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }

    pub fn set_y(&mut self, value: f32) {
        self.y = value;
    }
}
