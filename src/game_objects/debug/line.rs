use crate::{game_objects::game_object::DrawableObject, controller::position::Position, rendering::sprites::sprite_mapping::Sprite};


#[derive(Debug)]
pub enum Line{
    Horizontal{ position: Position},
    Vertical{ position: Position},
}

impl DrawableObject for Line{
    fn get_position(&self) -> Position {
        match self{
            Line::Horizontal{position} => *position,
            Line::Vertical{position} => *position,
        }
    }

    fn get_x_y_values(&self) -> (f32, f32) {
        match self{
            Line::Horizontal{position} => (position.x, position.y),
            Line::Vertical{position} => (position.x, position.y),
        }
    }

    fn get_size(&self) -> f32 {
        0.0
    }

    fn get_texture(&self) -> &crate::rendering::sprites::sprite_mapping::Sprite {
        &Sprite::Black
    }

    fn process_animation(&mut self, delta_time: f64) {
        
    }

    fn process_logic(&mut self, delta_time: f64) {
        
    }

    fn get_vertex_configuration(&self) -> &crate::rendering::sprites::vertex_configration::VertexConfigration {
       match self{
            Line::Horizontal{position: _} => &crate::rendering::sprites::vertex_configration::VertexConfigration::LINE_HORIZONTAL,
            Line::Vertical{position: _} => &crate::rendering::sprites::vertex_configration::VertexConfigration::LINE_VERTICAL,
        } 
    }
}