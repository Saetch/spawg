#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Debug)]
pub(crate) struct SpriteInstance{
    pub(crate) position: [f32;2],
    pub(crate) texture_id: u32,
}
impl SpriteInstance{
    pub(crate) fn new(position: [f32;2], texture_id: u32) -> Self{
        Self{
            position,
            texture_id,
        }
    }

    pub(crate) const fn desc() -> wgpu::VertexBufferLayout<'static>{
        wgpu::VertexBufferLayout{
            array_stride: std::mem::size_of::<SpriteInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute{
                    offset: 0,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute{
                    offset: std::mem::size_of::<[f32;2]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Uint32,
                }
            ]
        }
    }
}
