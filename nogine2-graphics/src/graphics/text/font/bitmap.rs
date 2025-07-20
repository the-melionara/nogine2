use std::collections::HashMap;

use nogine2_core::{crash, math::{rect::IRect, vector2::{ivec2, uvec2}}};

use crate::graphics::{text::rich::RichTextFunction, texture::sprite::{Sprite, SpriteAtlas}};

use super::{Font, FontCfg, TextStyle};

pub struct BitmapFont {
    styles: HashMap<TextStyle, StyledAtlasData>,
    cfg: FontCfg,
    rtf: Vec<Box<dyn RichTextFunction>>,
}

impl BitmapFont {
    /// Creates a new BitmapFont from a `SpriteAtlas` and a `charset`.
    /// For layout information, see the docs of `BitmapFont::set_style`.
    pub fn new(atlas: SpriteAtlas, charset: &str, cfg: FontCfg) -> Self {
        let mut res = Self { styles: HashMap::new(), cfg, rtf: Vec::new() };
        res.set_style(TextStyle::Regular, atlas, charset);
        return res;
    }

    /// Adds a new style.
    /// For correct usage, each `char` in `charset` must match to a cell in `atlas` in row-major
    /// order.
    pub fn set_style(&mut self, style: TextStyle, atlas: SpriteAtlas, charset: &str) {
        let width_in_cells = (atlas.tex().dims().0 / atlas.cell_size().0) as i32;
        let mut res = StyledAtlasData { atlas, rects: HashMap::new() };

        if self.cfg.monospace {
            for (i, c) in charset.chars().enumerate() {
                let pos = ivec2((i as i32) % width_in_cells, (i as i32) / width_in_cells);
                let rect = IRect { start: pos, end: pos + ivec2::ONE };
                res.rects.insert(c, rect);
            }
        } else {
            let cell_size = res.atlas.cell_size();
            res.atlas.normalize();
            for (i, c) in charset.chars().enumerate() {
                let pos = ivec2((i as i32) % width_in_cells, (i as i32) / width_in_cells);
                let Some(rect) = tight_fit(&res.atlas, pos, cell_size) else {
                    crash!("Can't access bitmap font pixel data!");
                };
                res.rects.insert(c, rect);
            }
        }

        self.styles.insert(style, res);
    }

    fn get_styled_char(&self, style: TextStyle, char: char) -> Option<Sprite> {
        let data = self.styles.get(&style)?;
        return Some(data.atlas.get(data.rects.get(&char).copied()?));
    }
}

fn tight_fit(atlas: &SpriteAtlas, pos: ivec2, cell_size: uvec2) -> Option<IRect> {
    let pos = uvec2::from(pos.scale(ivec2::from(cell_size)));
    let mut min = pos.0;
    let mut max = pos.0 + cell_size.0 - 1;

    let pixels = atlas.tex().pixels()?;
    
    // Left sweep
    'outer: for x in 0..cell_size.0 {
        for y in 0..cell_size.1 {
            if pixels.get_rgba8(pos + uvec2(x, y)).3 != 0 { // Non empty pixel
                min = pos.0 + x;
                break 'outer;
            }
        }
    }

    // Right sweep
    'outer: for x in (0..cell_size.0).rev() {
        for y in 0..cell_size.1 {
            if pixels.get_rgba8(pos + uvec2(x, y)).3 != 0 { // Non empty pixel
                max = pos.0 + x;
                break 'outer;
            }
        }
    }

    return Some(IRect {
        start: ivec2::from(uvec2(min, pos.1)),
        end: ivec2::from(uvec2(max + 1, pos.1 + cell_size.1))
    });
}

impl Font for BitmapFont {
    fn get_char(&self, mut style: TextStyle, char: char) -> Option<(Sprite, TextStyle)> {
        loop {
            match style {
                TextStyle::Regular => match self.get_styled_char(TextStyle::Regular, char) {
                    Some(x) => return Some((x, style)),
                    None => return None,
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

    fn cfg(&self) -> &FontCfg {
        &self.cfg
    }

    fn get_rich_functions(&self) -> &[Box<dyn RichTextFunction>] {
        &self.rtf
    }
}

struct StyledAtlasData {
    atlas: SpriteAtlas,
    rects: HashMap<char, IRect>,
}
