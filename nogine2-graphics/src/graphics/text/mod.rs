use std::collections::HashMap;

use nogine2_core::{crash, math::{rect::IRect, vector2::ivec2}};

use super::texture::sprite::{Sprite, SpriteAtlas};

pub struct BitmapFont {
    styles: HashMap<TextStyle, StyledAtlasData>,
    monospace: bool,
}

impl BitmapFont {
    /// Creates a new BitmapFont from a `SpriteAtlas` and a `charset`.
    /// For layout information, see the docs of `BitmapFont::set_style`.
    pub fn new(atlas: SpriteAtlas, charset: &str, monospace: bool) -> Self {
        let mut res = Self { styles: HashMap::new(), monospace };
        res.set_style(TextStyle::Regular, atlas, charset);
        return res;
    }

    /// Adds a new style.
    /// For correct usage, each `char` in `charset` must match to a cell in `atlas` in row-major
    /// order.
    pub fn set_style(&mut self, style: TextStyle, atlas: SpriteAtlas, charset: &str) {
        let width_in_cells = (atlas.tex().dims().0 / atlas.cell_size().0) as i32;
        let mut res = StyledAtlasData { atlas, rects: HashMap::new() };

        if self.monospace {
            for (i, c) in charset.chars().enumerate() {
                let pos = ivec2((i as i32) % width_in_cells, (i as i32) /  width_in_cells);
                let rect = IRect { start: pos, end: pos + ivec2::ONE };
                res.rects.insert(c, rect);
            }
        } else {
            unimplemented!()
        }

        self.styles.insert(style, res);
    }

    /// Returns the sprite of a char given a style. If the character is not availoable in a style,
    /// a fallback will be searched.
    pub fn get_char(&self, mut style: TextStyle, char: char) -> Option<(Sprite, TextStyle)> {
        loop {
            match style {
                TextStyle::Regular => match self.get_styled_char(TextStyle::Regular, char) {
                    Some(x) => return Some((x, style)),
                    None => crash!("All fonts should have a regular style. This shouldn't even
                        happen!"),
                },
                TextStyle::Bold => match self.get_styled_char(TextStyle::Bold, char) {
                    Some(x) => return Some((x, style)),
                    None => style = TextStyle::Regular,
                },
                TextStyle::Italic => match self.get_styled_char(TextStyle::Italic, char) {
                    Some(x) => return Some((x, style)),
                    None => style = TextStyle::Regular,
                },
                TextStyle::BoldItalic => match self.get_styled_char(TextStyle::BoldItalic, char) {
                    Some(x) => return Some((x, style)),
                    None => style = TextStyle::Bold,
                },
            }
        }
    }

    fn get_styled_char(&self, style: TextStyle, char: char) -> Option<Sprite> {
        let data = self.styles.get(&style)?;
        return Some(data.atlas.get(data.rects.get(&char).copied()?));
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TextStyle {
    Regular, Bold, Italic, BoldItalic,
}

struct StyledAtlasData {
    atlas: SpriteAtlas,
    rects: HashMap<char, IRect>,
}
