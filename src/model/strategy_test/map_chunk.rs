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

