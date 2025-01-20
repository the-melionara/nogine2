use area::UIArea;
use nogine2_core::math::vector2::vec2;

pub mod area;
pub mod layout;

pub type UIHash = hash::FNV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Anchor {
     LeftUp,   Up,   RightUp,
      Left,  Center,  Right,
    LeftDown, Down, RightDown,
}

impl Anchor {
    pub(crate) fn local_pivot(&self) -> vec2 {
        match self {
            Anchor::LeftUp => vec2(0.0, 0.0),
            Anchor::Up => vec2(0.5, 0.0),
            Anchor::RightUp => vec2(1.0, 0.0),
            Anchor::Left => vec2(0.0, 0.5),
            Anchor::Center => vec2(0.5, 0.5),
            Anchor::Right => vec2(1.0, 0.5),
            Anchor::LeftDown => vec2(0.0, 1.0),
            Anchor::Down => vec2(0.5, 1.0),
            Anchor::RightDown => vec2(1.0, 1.0),
        }
    }
}

pub trait UIWidget<'a> {
    /// Returns an iterator over all the data that will remain unique among other UI elements inside the same area and consistent across frames (in byte slice form). This is required to assign unique IDs to elements of this type.
    fn unique_data(&self) -> &[u8];
    fn set_id(&mut self, id: UIHash);
    fn id(&self) -> UIHash;

    type RunRet;
    fn run(self, parent: &UIArea<'a>) -> Self::RunRet;
}
    
mod hash {
    use std::num::Wrapping;

    pub type FNV1 = Wrapping<u64>;

    const FNV_OFFSET_BASIS: FNV1 = Wrapping(0xCBF29CE484222325);
    const FNV_PRIME: FNV1 = Wrapping(0x100000001B3);

    // https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function#FNV-1_hash
    pub fn fnv1<'a>(parent_id: FNV1, bytes: &'a [u8]) -> FNV1 {
        let mut hash = FNV_OFFSET_BASIS;
        for b in &parent_id.0.to_ne_bytes() {
            hash *= FNV_PRIME;
            hash ^= Wrapping(*b as u64);
        }
 
        for b in bytes {
            hash *= FNV_PRIME;
            hash ^= Wrapping(*b as u64);
        }
        return hash;
    }
}
