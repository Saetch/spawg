use crate::{rendering::sprites::{sprite_mapping::Sprite, vertex_configration::VertexConfigration}, controller::position::Position, game_objects::game_object::DrawableObject, model::{results::LogicResult, strategy_test::{strategy_test::StrategyLogicObject, map_chunk::MapChunk}}};

#[derive(Debug)]
pub(crate) struct StartObj{
    pub(crate) texture: Sprite,
    pub position: Position,
    pub vertices: VertexConfigration,
    pub wares: Vec<WareAmount>,
    id : u64,
}

#[derive(Debug)]
pub(crate) struct WareAmount{
    pub(crate) ware_type: u32,
    pub(crate) amount: u32,
}

impl StartObj{
    pub(crate) fn new(position: Position, id: u64) -> Self{
        Self{
            texture: Sprite::BaseBaseLarge,
            position,
            vertices: VertexConfigration::BIG_BASE_BASE_4X2,
            wares: Vec::new(),
            id,
        }
    }
}

impl StrategyLogicObject for StartObj{
    fn process_logic(&mut self, delta_time: std::time::Duration, _blockers: &mut Vec<Box<dyn MapChunk>>, _structures: &mut Vec<Box<dyn crate::game_objects::game_object::VisitableStructure>>) -> LogicResult {
        LogicResult::None
    }


    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn get_id(&self) -> u64 {
        self.id
    }

}

impl DrawableObject for StartObj{
    fn get_position(&self) -> Position {
        self.position
    }


    fn get_x_y_values(&self) -> (f32, f32) {
        (self.position.x, self.position.y)
    }

    fn get_size(&self) -> f32 {
        todo!()
    }

    fn get_texture(&self) -> &Sprite {
        &self.texture
    }

    fn process_animation(&mut self, delta_time: f64) {
    }

    fn get_vertex_configuration(&self) -> &VertexConfigration {
        &self.vertices
    }

    fn get_id(&self) -> u64 {
        self.id
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    


}