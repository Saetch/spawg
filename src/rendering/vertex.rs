

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
}


impl Vertex {
    pub const fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress, //this tells the GPU how large a single vertex is
            step_mode: wgpu::VertexStepMode::Vertex, //interpret data as individual vertices
            attributes: &[ //describe the layout of a single vertex
                wgpu::VertexAttribute {
                    offset: 0, //offset in memory, thus for the first it is zero and later it is size of the previous attributes
                    shader_location: 0, //corresponds to the location(=index) of the attribute in the vertex shader, 0 is usually the position, 1 being the color
                    format: wgpu::VertexFormat::Float32x2, //tells the GPU what type of data we input, in this case a Vec3<f32>
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                }
            ]
        }
    }
}