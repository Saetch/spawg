#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Debug)]
pub(crate) struct SpriteInstance{
    pub(crate) position: [f32;2],
    pub(crate) texture_id: u32,
}