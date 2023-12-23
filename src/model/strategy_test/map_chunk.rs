use std::fmt;

use crate::controller::position::Position;

#[derive(Debug)]
pub(crate) struct IntEdge{
    x: i32,
    y: i32,
}
#[derive(Debug)]
pub(crate) struct ChunkInfo{
    bottom_left: IntEdge,
    top_right: IntEdge,
}
pub(crate) trait MapChunk: fmt::Debug{
    
    fn identify(&self) -> String;
    fn inf(&self) -> ChunkInfo;
}

