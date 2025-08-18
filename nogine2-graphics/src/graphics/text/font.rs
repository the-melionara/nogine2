use super::{super::texture::sprite::Sprite, rich::RichTextFunction};

pub mod bitmap;

pub trait Font {
    /// Returns the sprite of a char given a style. If the character is not availoable in a style,
    /// a fallback will be searched.
    fn get_char(&self, style: TextStyle, char: char) -> Option<(Sprite, TextStyle)>;
    fn cfg(&self) -> &FontCfg;
    fn get_rich_functions(&self) -> &[Box<dyn RichTextFunction>];
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TextStyle {
    Regular, Bold, Italic, BoldItalic,
}

pub struct FontCfg {
    pub monospace: bool,

    /// Measure::Percent is relative to the font size when rendering the text.
    pub space_width: Measure,

    /// Measure::Percent is relative to the font size when rendering the text.
    pub char_separation: Measure,

    /// Measure::Percent is relative to the font size when rendering the text.
    pub line_separation: Measure,
}

#[derive(Debug, Clone, Copy)]
pub enum Measure {
    /// Percent (0 to 1) of some other metric.
    Percent(f32),
    /// Pixels in relation to the render target.
    Pixels(f32),
}
