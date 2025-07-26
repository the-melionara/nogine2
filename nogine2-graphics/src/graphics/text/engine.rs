use std::{iter::Peekable, str::CharIndices, sync::Arc};

use helpers::GraphicMetrics;
use nogine2_core::{assert_expr, crash, log_warn, math::{mat3x3::mat3, rect::Rect, vector2::vec2, vector3::vec3}};

use crate::{colors::{rgba::RGBA32, Color}, graphics::{batch::{BatchData, BatchPushCmd}, blending::BlendingMode, material::Material, texture::{sprite::Sprite, TextureHandle, TextureWrapping}, vertex::BatchVertex}};

use super::{font::{Font, TextStyle}, TextCfg};

pub struct TextEngine {
    cursor: vec2,
    extents: vec2,

    // Given how much text is used, these types must NOT be reallocated. I don't want to waste
    // precious cycles allocating and reallocating buffers in performance sensitive code.
    // If they already hold the allocated space I need they, shall serve me well.
    batches: Vec<TextBatch>,
    text_buf: String,
    lines_buf: Vec<LineData>,

    rtf_stack: Vec<RTCmd>,
    rtf_args_stack: String,
}

impl TextEngine {
    pub const fn new() -> Self {
        return Self {
            cursor: vec2::ZERO,
            extents: vec2::ZERO,
            
            // These things must only be created HERE!!!! NO RECREATIONS ELSEWHERE.
            batches: Vec::new(),
            text_buf: String::new(),
            lines_buf: Vec::new(),

            rtf_stack: Vec::new(),
            rtf_args_stack: String::new(),
        }
    }

    pub fn add_sprite(&mut self, offset: vec2, sprite: &Sprite, scale: f32) {
        let rect = Rect::from_points(
            offset,
            offset + vec2::from(sprite.dims()) * scale,
        );
        
        // THIS MAY NEED REVISIONS!!!!!!!
        // if rect.left() < 0.0 || rect.right() > self.extents.0 ||
        //     rect.down() < 0.0 || rect.up() > self.extents.1 {
        //     return;
        // }
        
        self.batches.push(TextBatch {
            verts: [rect.ld(), rect.lu(), rect.ru(), rect.rd()],
            offset: self.cursor - rect.size().yvec(),
            uvs: sprite.uv_rect(),
            texture: sprite.handle().clone(),
        });
    }

    pub fn advance_x(&mut self, dx: f32) {
        self.cursor.0 += dx;
    }

    pub fn advance_y(&mut self, dy: f32) {
        self.cursor.1 -= dy;
        self.cursor.0 = 0.0;
    }

    pub fn render(
        &self,
        batch_data: &mut BatchData,
        transform: mat3,
        culling_enabled: bool,
        blending: BlendingMode,
        material: Arc<Material>,
    ) {
        for b in &self.batches {
            batch_data.push(
                BatchPushCmd::Triangles {
                    verts: &[
                        BatchVertex {
                            pos: (&transform * vec3::from_xy(b.offset + b.verts[0], 1.0)).xy(),
                            tint: RGBA32::WHITE,
                            uv: b.uvs.lu(),
                            tex_id: 0,
                            user_data: 0,
                        },
                        BatchVertex {
                            pos: (&transform * vec3::from_xy(b.offset + b.verts[1], 1.0)).xy(),
                            tint: RGBA32::WHITE,
                            uv: b.uvs.ld(),
                            tex_id: 0,
                            user_data: 0,
                        },
                        BatchVertex {
                            pos: (&transform * vec3::from_xy(b.offset + b.verts[2], 1.0)).xy(),
                            tint: RGBA32::WHITE,
                            uv: b.uvs.rd(),
                            tex_id: 0,
                            user_data: 0,
                        },
                        BatchVertex {
                            pos: (&transform * vec3::from_xy(b.offset + b.verts[3], 1.0)).xy(),
                            tint: RGBA32::WHITE,
                            uv: b.uvs.ru(),
                            tex_id: 0,
                            user_data: 0,
                        },
                    ],
                    indices: &[0, 1, 2, 2, 3, 0],
                    texture: b.texture.clone(),
                    blending,
                    material: material.clone()
                },
                culling_enabled
            );
        }
    }

    pub fn load(&mut self, text: &str, cfg: &TextCfg<'_>, tex_ppu: f32) {
        self.batches.clear();
        self.cursor = vec2(0.0, self.extents.1);
        self.extents = cfg.extents;

        self.lines_buf.clear();
        self.text_buf.clear();
        self.rtf_clear();

        let mut lines_swap = Vec::new();
        let mut text_swap = String::new();

        std::mem::swap(&mut self.text_buf, &mut text_swap);
        std::mem::swap(&mut self.lines_buf, &mut lines_swap);
        let mut gear = EngineGear::new(text, &mut text_swap, &mut lines_swap);

        let GraphicMetrics {
            line_height,
            char_separation,
            space_width: space_char_width,
        } = GraphicMetrics::calculate(cfg, tex_ppu);
        
        let mut escaped_tag = false;
        let mut iter = text.char_indices().peekable();
        while let Some((i, c)) = iter.next() {
            match c {
                '\r' => continue, // I have little interest in DEVILISH newline characters
                '\n' => gear.pop_line(), // I do have interest in REAL newline characters
                _ if c.is_whitespace() => gear.push_space(c, i, space_char_width),
                _ => {
                    // Rich text tag
                    if cfg.rich_text && c == '<' && !escaped_tag {
                        let mut closing = false;
                        match iter.peek().copied() {
                            Some((_, '<')) => { // Escaped '<'
                                escaped_tag = true;
                                continue;
                            }, 
                            Some((_, '/')) => { // Closing tag
                                closing = true;
                                iter.next();
                            }
                            _ => {}
                        }

                        if let Some((name, args)) = process_tag(text, &mut iter, closing, i, c) {
                            self.rtf_push(name, args, cfg.font);
                        }

                        if closing {
                            self.rtf_pop();
                        }

                        continue;
                    }
                    escaped_tag = false;

                    if let Some((sprite, _)) = cfg.font.get_char(TextStyle::Regular, c) {
                        let width = sprite.dims().0 as f32 / sprite.dims().1 as f32 * line_height;

                        gear.push_char(c, i, width, char_separation);

                        // Word wrap!
                        if cfg.word_wrap && gear.to_be_wrapper(cfg.extents.0) {
                            gear.wrap_line();
                        }
                    }
                },
            }
        }

        gear.finalize();
        std::mem::swap(&mut self.text_buf, &mut text_swap);
        std::mem::swap(&mut self.lines_buf, &mut lines_swap);
    }

    /// This method exists EXCLUSIVELY BECAUSE I HATE THE BORROW CHECKER.
    pub fn swap_sanitized_text(&mut self, res: &mut String) {
        std::mem::swap(&mut self.text_buf, res);
    }

    pub fn get_line_data(&self, index: usize) -> LineData {
        self.lines_buf[index]
    }

    pub fn get_line_count(&self) -> usize {
        self.lines_buf.len()
    }

    fn rtf_clear(&mut self) {
        self.rtf_stack.clear();
        self.rtf_args_stack.clear();
    }

    fn rtf_push(&mut self, name: &str, args: &str, font: &dyn Font) {
        let rtfs = font.get_rich_functions();
        let Some(index) = rtfs.iter().position(|x| x.get_tag_name().trim() == name.trim()) else {
            log_warn!("{name} is not a recognized rich text function.");
            return;
        };

        self.rtf_stack.push(RTCmd { index: Some(index), char_index: self.text_buf.len() });
        self.rtf_args_stack.push_str(args);
        self.rtf_args_stack.push('\n');
    }

    fn rtf_pop(&mut self) {
        self.rtf_stack.push(RTCmd { index: None, char_index: self.text_buf.len() });
        self.rtf_args_stack.push('\n');
    }
}

fn process_tag<'a>(
    src: &'a str,
    iter: &mut Peekable<CharIndices<'_>>,
    closing_tag: bool,
    i: usize,
    c: char
) -> Option<(&'a str, &'a str)> {
    if closing_tag {
        while iter.next_if(|(_, c)| *c != '>').is_some() {}
        iter.next();
        return None;
    }
    
    // Name
    let mut name_start = i + c.len_utf8();
    while let Some((i, _)) = iter.next_if(|(_, c)| c.is_whitespace()) {
        name_start = i;
    }

    let mut name_end = name_start;
    while let Some((i, c)) = iter.next_if(|(_, c)| !matches!(c, '=' | '>')) {
        if c == '\n' {
            return None;
        }
        
        name_end = i + c.len_utf8();
    }
    let name = &src[name_start..name_end];

    // Midpoint
    while iter.next_if(|(_, c)| c.is_whitespace()).is_some() {}
    let args_start = match iter.next() {
        Some((_, '>')) => return Some((name, "")),
        Some((i, '=')) => i + '='.len_utf8(),
        Some(_) => crash!("SHOULDN'T HAPPEN 1997"),
        None => return None,
    };

    // Args
    let mut args_end = args_start;
    while let Some((i, c)) = iter.next_if(|(_, c)| !matches!(c, '>' | '\n')) {
        args_end = i + c.len_utf8();
    }
    let args = &src[args_start..args_end];

    iter.next();
    return Some((name, args));
}

struct RTCmd {
    /// `None` for pop
    index: Option<usize>,
    char_index: usize,
}

/// As I don't have a better name, I will call this a 'Gear' because it sounds rad as fuck.
/// It's basically the thing that makes the sanitizer spin.
struct EngineGear<'a> {
    src: &'a str,
    text_buf: &'a mut String,
    lines_buf: &'a mut Vec<LineData>,
    
    word_range: (usize, usize),
    space_range: (usize, usize),

    line_data: LineData,
    word_width: f32,
    space_width: f32,

    on_word: bool,
}

impl<'a> EngineGear<'a> {
    fn new(src: &'a str, text_buf: &'a mut String, lines_buf: &'a mut Vec<LineData>) -> Self {
        return Self {
            src,
            text_buf,
            lines_buf,
            word_range: (0, 0),
            space_range: (0, 0),
            line_data: LineData::new(),
            word_width: 0.0,
            space_width: 0.0,
            on_word: true,
        };
    }

    fn pop_line(&mut self) {
        self.push_batch();
        self.lines_buf.push(self.line_data);

        self.line_data = LineData::new();
        self.on_word = false;
        
        self.text_buf.push('\n');
    }

    fn wrap_line(&mut self) {
        self.line_data.word_wrapped = true;
        self.lines_buf.push(self.line_data);

        self.line_data = LineData::new();
        self.space_range = (0, 0);
        self.space_width = 0.0;
        
        self.text_buf.push('\n');
    }

    fn push_space(&mut self, char: char, index: usize, space_char_width: f32) {
        // Word and prev space should be applied
        if self.on_word {
            self.push_batch();
        }

        self.space_width += space_char_width;
        self.space_range = (
            // For intentional spaces at the start of a line
            if self.on_word || self.current_spaces() == 0 {
                index
            } else {
                self.space_range.0
            },
            index + char.len_utf8(),
        );
        self.on_word = false;
    }

    fn push_char(&mut self, char: char, index: usize, width: f32, char_separation: f32) {
        self.word_range = (
            if !self.on_word {
                self.word_width = 0.0;
                index
            } else {
                self.word_range.0
            },
            index + char.len_utf8(),
        );
        
        self.word_width += width + char_separation;
        self.on_word = true;
    }

    fn push_batch(&mut self) -> bool {
        let mut res = false;
        if self.space_range.0 != self.space_range.1 {
            self.text_buf.push_str(&self.src[self.space_range.0..self.space_range.1]);
            self.line_data.space_count += self.current_spaces();
            self.line_data.min_width += self.space_width;

            self.space_width = 0.0;
            self.space_range = (0, 0);
            res = true;
        }

        if self.word_range.0 != self.word_range.1 {
            self.text_buf.push_str(&self.src[self.word_range.0..self.word_range.1]);
            self.line_data.min_width += self.word_width;
            self.line_data.spaceless_width += self.word_width;

            self.word_width = 0.0;
            self.word_range = (0, 0);
            res = true;
        }
        return res;
    }

    fn finalize(&mut self) {
        if self.push_batch() {
            self.lines_buf.push(self.line_data);
        }
    }

    fn to_be_wrapper(&self, extents_width: f32) -> bool {
        return self.line_data.min_width + self.word_width + self.space_width > extents_width
            && self.line_data.min_width > 0.0; // so one word lines don't get skipped
    }

    #[inline] fn current_spaces(&self) -> u32 {
        (self.space_range.1 - self.space_range.0) as u32
    }
}

pub struct TextBatch {
    verts: [vec2; 4], // ld, lu, ru, rd
    offset: vec2,
    uvs: Rect,
    texture: TextureHandle,
}

#[derive(Debug, Clone, Copy)]
pub struct LineData {
    pub min_width: f32,
    pub spaceless_width: f32,
    pub word_wrapped: bool,
    pub space_count: u32,
}

impl LineData {
    pub const fn new() -> Self {
        Self { min_width: 0.0, spaceless_width: 0.0, word_wrapped: false, space_count: 0 }
    }
}

pub mod helpers {
    use crate::graphics::text::{font::Measure, TextCfg};

    pub struct GraphicMetrics {
        pub line_height: f32,
        pub char_separation: f32,
        pub space_width: f32,
    }

    impl GraphicMetrics {
        /// `space_width` already includes `char_separation`
        pub fn calculate(cfg: &TextCfg<'_>, tex_ppu: f32) -> Self {
            let line_height = cfg.font_size / tex_ppu;
            let char_separation = match cfg.font.cfg().char_separation {
                Measure::Percent(x) => x * line_height,
                Measure::Pixels(x) => x / tex_ppu,
            };
            let space_width = match cfg.font.cfg().space_width {
                Measure::Percent(x) => x * line_height,
                Measure::Pixels(x) => x / tex_ppu,
            } + char_separation;

            return Self { line_height, char_separation, space_width };
        }
    }
}
