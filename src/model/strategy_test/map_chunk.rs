use crate::controller::position::Position;


pub(crate) struct IntEdge{
    x: i32,
    y: i32,
}
pub(crate) struct ChunkInfo{
    bottom_left: IntEdge,
    top_right: IntEdge,
}
pub(crate) trait MapChunk {
    
    fn identify(&self) -> String;
    fn inf(&self) -> ChunkInfo;
}

