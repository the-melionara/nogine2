use font::Font;
use nogine2_core::math::rect::Rect;

pub mod font;
pub(crate) mod engine;

pub struct TextCfg<'a> {
    pub bounds: Rect,
    pub font_size: f32,
    pub font: &'a dyn Font,
}
