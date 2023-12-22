use super::vertex_configration::VertexConfigrationTrait;




struct DynamicVertexConfiguration{
    data: [crate::rendering::vertex::Vertex; 4],
}

impl VertexConfigrationTrait for DynamicVertexConfiguration{
    fn get_vertices(&self) -> [crate::rendering::vertex::Vertex; 4] {
        return self.data;
    }
}