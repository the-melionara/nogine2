use std::sync::Arc;

use nogine2_core::math::{rect::Rect, vector2::vec2};

use crate::{colors::{rgba::RGBA32, Color}, graphics::{batch::{BatchData, BatchPushCmd}, blending::BlendingMode, material::Material, texture::{sprite::Sprite, TextureHandle}, vertex::BatchVertex}};

pub struct TextEngine {
    batches: Vec<TextBatch>,
    cursor: vec2,
}

impl TextEngine {
    pub const fn new() -> Self {
        return Self {
            batches: Vec::new(),
            cursor: vec2::ZERO,
        }
    }

    pub fn add_sprite(&mut self, offset: vec2, sprite: &Sprite, scale: f32) {
        self.batches.push(TextBatch {
            verts: [
                self.cursor + offset,
                self.cursor + offset + vec2::from(sprite.dims().yvec()) * scale,
                self.cursor + offset + vec2::from(sprite.dims()) * scale,
                self.cursor + offset + vec2::from(sprite.dims().xvec()) * scale,
            ],
            uvs: sprite.uv_rect(),
            texture: sprite.handle().clone(),
        });
    }

    pub fn advance_x(&mut self, dx: f32) {
        self.cursor.0 += dx;
    }

    pub fn render(
        &self,
        batch_data: &mut BatchData,
        culling_enabled: bool,
        blending: BlendingMode,
        material: Arc<Material>,
    ) {
        for b in &self.batches {
            batch_data.push(
                BatchPushCmd::Triangles {
                    verts: &[
                        BatchVertex {
                            pos: b.verts[0],
                            tint: RGBA32::WHITE,
                            uv: b.uvs.lu(),
                            tex_id: 0,
                            user_data: 0,
                        },
                        BatchVertex {
                            pos: b.verts[1],
                            tint: RGBA32::WHITE,
                            uv: b.uvs.ld(),
                            tex_id: 0,
                            user_data: 0,
                        },
                        BatchVertex {
                            pos: b.verts[2],
                            tint: RGBA32::WHITE,
                            uv: b.uvs.rd(),
                            tex_id: 0,
                            user_data: 0,
                        },
                        BatchVertex {
                            pos: b.verts[3],
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

    pub fn reset(&mut self) {
        self.batches.clear();
        self.cursor = vec2::ZERO;
    }
}

pub struct TextBatch {
    verts: [vec2; 4], // ld, lu, ru, rd
    uvs: Rect,
    texture: TextureHandle,
}
