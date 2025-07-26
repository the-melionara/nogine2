use align::{HorTextAlign, VerTextAlign};
use font::Font;
use nogine2_core::math::vector2::vec2;

pub mod font;
pub mod align;
pub mod rich;
pub(crate) mod engine;

pub struct TextCfg<'a> {
    pub origin: vec2,
    pub rot: f32,
    pub scale: vec2,
    pub extents: vec2,
    pub font_size: f32,
    pub font: &'a dyn Font,
    pub hor_alignment: HorTextAlign,
    pub ver_alignment: VerTextAlign,
    pub word_wrap: bool,
    pub rich_text: bool,
}
