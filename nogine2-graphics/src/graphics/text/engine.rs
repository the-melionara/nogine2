use std::sync::Arc;

use helpers::GraphicMetrics;
use nogine2_core::math::{mat3x3::mat3, rect::Rect, vector2::vec2, vector3::vec3};

use crate::{colors::{rgba::RGBA32, Color}, graphics::{batch::{BatchData, BatchPushCmd}, blending::BlendingMode, material::Material, texture::{sprite::Sprite, TextureHandle}, vertex::BatchVertex}};

use super::{font::TextStyle, TextCfg};

pub struct TextEngine {
    cursor: vec2,
    extents: vec2,

    // Given how much text is used, these types must NOT be reallocated. I don't want to waste
    // precious cycles allocating and reallocating buffers in performance sensitive code.
    // If they already hold the allocated space I need they, shall serve me well.
    batches: Vec<TextBatch>,
    text_buf: String,
    lines_buf: Vec<LineData>,
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

        let mut line_start = 0;
        let mut line_end = 0;
        let mut line_data = LineData::new();

        let GraphicMetrics {
            line_height,
            char_separation,
            space_width
        } = GraphicMetrics::calculate(cfg, tex_ppu);
        
        for (i, c) in text.char_indices() {
            match c {
                '\r' => continue, // I have little interest in DEVILISH newline characters
                '\n' => { // I do have interest in REAL newline characters
                    let slice = &text[line_start..line_end];
                    self.text_buf.push_str(slice);
                    self.text_buf.push('\n');

                    self.lines_buf.push(line_data);

                    line_start = i + c.len_utf8();
                    line_end = line_start;
                    line_data = LineData::new();
                }
                _ => {
                    line_end = i + c.len_utf8();

                    if c.is_whitespace() {
                        let dx = 2.0 * char_separation + space_width;
                        line_data.min_width += dx;
                    } else if let Some((sprite, _)) = cfg.font.get_char(TextStyle::Regular, c) {
                        let width = sprite.dims().0 as f32 / sprite.dims().1 as f32 * line_height;
                        let dx = width + char_separation;

                        line_data.min_width += dx;
                        line_data.spaceless_width += dx;
                    }
                },
            }
        }

        if line_start != line_end {
            let slice = &text[line_start..line_end];
            self.text_buf.push_str(slice);
            self.lines_buf.push(line_data);
        }
    }

    /// This method exists EXCLUSIVELY BECAUSE I HATE THE BORROW CHECKER.
    pub fn swap_sanitized_text(&mut self, res: &mut String) {
        std::mem::swap(&mut self.text_buf, res);
    }

    pub fn get_line_data(&self, index: usize) -> LineData {
        self.lines_buf[index]
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
}

impl LineData {
    pub const fn new() -> Self {
        Self { min_width: 0.0, spaceless_width: 0.0 }
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
        pub fn calculate(cfg: &TextCfg<'_>, tex_ppu: f32) -> Self {
            let line_height = cfg.font_size / tex_ppu;
            let char_separation = match cfg.font.cfg().char_separation {
                Measure::Percent(x) => x * line_height,
                Measure::Pixels(x) => x / tex_ppu,
            };
            let space_width = match cfg.font.cfg().space_width {
                Measure::Percent(x) => x * line_height,
                Measure::Pixels(x) => x / tex_ppu,
            };

            return Self { line_height, char_separation, space_width };
        }
    }
}
