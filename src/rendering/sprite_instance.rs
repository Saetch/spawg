#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct SpriteInstance{
    pub(crate) position: (f32, f32),
    pub(crate) texture_id: u32,
}