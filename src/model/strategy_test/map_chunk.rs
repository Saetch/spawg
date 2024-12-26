use std::fmt;

use crate::controller::position::Position;

#[derive(Debug)]
pub(crate) struct IntEdge{
    pub(crate) x: i32,
    pub(crate) y: i32,
}
#[derive(Debug)]
pub(crate) struct ChunkInfo{
    pub(crate) bottom_left: IntEdge,
    pub(crate) top_right: IntEdge,
}
pub(crate) trait MapChunk: fmt::Debug{
    
    fn identify(&self) -> String;
    fn inf(&self) -> ChunkInfo;
}

impl IntEdge{
    pub(crate) fn new(x: i32, y: i32) -> Self{
        Self{
            x,
            y,
        }
    }
}

impl ChunkInfo{
    pub(crate) fn new(x: f32, y: f32, width: f32, height: f32) -> Self{
        let bottom_left = IntEdge::new((x - width / 2.0) as i32, (y - height / 2.0) as i32);
        let top_right = IntEdge::new((x + width / 2.0) as i32, (y + height / 2.0) as i32);
        Self{
            bottom_left,
            top_right,
        }
    }

    pub(crate) fn contains(&self, position: &Position) -> bool{
        let x = position.x as i32;
        let y = position.y as i32;
        x >= self.bottom_left.x && x <= self.top_right.x && y >= self.bottom_left.y && y <= self.top_right.y
    }
}

