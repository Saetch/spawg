use crate::{game_objects::game_object::DrawableObject, controller::position::Position, rendering::sprites::sprite_mapping::Sprite, model::results::LogicResult};


#[derive(Debug)]
pub enum Line{
    Horizontal{ position: Position, id: u64},
    Vertical{ position: Position, id: u64},
}

impl DrawableObject for Line{
    fn get_position(&self) -> Position {
        match self{
            Line::Horizontal{position, id: _} => *position,
            Line::Vertical{position, id: _} => *position,
        }
    }

    fn get_x_y_values(&self) -> (f32, f32) {
        match self{
            Line::Horizontal{position, id: _} => (position.x, position.y),
            Line::Vertical{position, id: _} => (position.x, position.y),
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

    #[inline(always)]
    fn process_logic(&mut self, delta_time: f64) -> LogicResult{
        return LogicResult::None;
    }

    fn get_vertex_configuration(&self) -> &crate::rendering::sprites::vertex_configration::VertexConfigration {
       match self{
            Line::Horizontal{position: _, id: _} => &crate::rendering::sprites::vertex_configration::VertexConfigration::LINE_HORIZONTAL,
            Line::Vertical{position: _, id: _} => &crate::rendering::sprites::vertex_configration::VertexConfigration::LINE_VERTICAL,
        } 
    }

    fn get_id(&self) -> u64 {
        match self{
            Line::Horizontal{position: _, id: i} => *i,
            Line::Vertical{position: _, id: i} => *i,
        } 
    }

    fn set_id(&mut self, id: u64) {
        match self{
            Line::Horizontal{position: _, id: i} => *i = id,
            Line::Vertical{position: _, id: i} => *i = id,
        }
        }
}
