use std::{marker::PhantomData, str::Split};

use nogine2_core::math::vector2::vec2;

use crate::colors::rgba::RGBA32;

use super::font::TextStyle;

pub trait RichTextFunction {
    /// Returns the name of the tag associated with this function.
    fn get_tag_name(&self) -> &'static str;

    /// Defines whether `draw` is executed only once (`true`) or per character (`false`).
    fn is_event(&self) -> bool { false }
    
    /// Main function to override. Defines how the character drawing is modified.
    /// `out_quads` is prefilled with the data in `ìn_quads`.
    fn draw(
        &self,
        args: Split<'_, char>,
        render_data: &mut CharRenderData,
        in_quads: &[CharQuad],
        out_quads: &mut Vec<CharQuad>,
        ctx: &RichTextContext
    );
}

pub struct RichTextContext {
    /// Current game timestamp.
    pub time: f32,
    /// Timestep.
    pub ts: f32,
    /// Index of character being processed.
    pub index: usize,
    /// Character being processed.
    pub char: char,
}

pub struct CharRenderData {
    pub style: TextStyle,

    /// Just to fuck with whoever tries to create this instead of modifying it
    _phantom: PhantomData<()>,
}

#[derive(Debug, Clone, Copy)]
pub struct CharQuad {
    pub lu: CharVert,
    pub ld: CharVert,
    pub ru: CharVert,
    pub rd: CharVert,
}

#[derive(Debug, Clone, Copy)]
pub struct CharVert {
    pub pos: vec2,
    pub color: RGBA32,
    pub user_data: i32,
}
