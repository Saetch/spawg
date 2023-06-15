use std::sync::{Arc, RwLock};

pub struct Position {
    x: Arc<RwLock<f32>>,
    y: Arc<RwLock<f32>>,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        CamPosition {
            x: Arc::new(RwLock::new(x)),
            y: Arc::new(RwLock::new(y)),
        }
    }

    pub fn get_x(&self) -> f32 {
        *self.x.read().unwrap()
    }

    pub fn set_x(&self, value: f32) {
        *self.x.write().unwrap() = value;
    }

    pub fn get_y(&self) -> f32 {
        *self.y.read().unwrap()
    }

    pub fn set_y(&self, value: f32) {
        *self.y.write().unwrap() = value;
    }
}
