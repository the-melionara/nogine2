use std::sync::Arc;

use nogine2_core::math::{mat3x3::mat3, rect::Rect, vector2::vec2, vector3::vec3};

use crate::{colors::{rgba::RGBA32, Color}, graphics::{batch::{BatchData, BatchPushCmd}, blending::BlendingMode, material::Material, texture::{sprite::{self, Sprite}, TextureHandle}, vertex::BatchVertex}};

pub struct TextEngine {
    batches: Vec<TextBatch>,
    cursor: vec2,
    extents: vec2,
}

impl TextEngine {
    pub const fn new() -> Self {
        return Self {
            batches: Vec::new(),
            cursor: vec2::ZERO,
            extents: vec2::ZERO,
        }
    }

    pub fn add_sprite(&mut self, offset: vec2, sprite: &Sprite, scale: f32) {
        let rect = Rect {
            start: self.cursor + offset,
            end: self.cursor + offset + vec2::from(sprite.dims()) * scale,
        };

        if rect.left() < 0.0 || rect.right() > self.extents.0 ||
            rect.down() < 0.0 || rect.up() > self.extents.1 {
            return;
        }
        
        self.batches.push(TextBatch {
            verts: [rect.ld(), rect.lu(), rect.ru(), rect.rd()],
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
                            pos: (&transform * vec3::from_xy(b.verts[0], 1.0)).xy(),
                            tint: RGBA32::WHITE,
                            uv: b.uvs.lu(),
                            tex_id: 0,
                            user_data: 0,
                        },
                        BatchVertex {
                            pos: (&transform * vec3::from_xy(b.verts[1], 1.0)).xy(),
                            tint: RGBA32::WHITE,
                            uv: b.uvs.ld(),
                            tex_id: 0,
                            user_data: 0,
                        },
                        BatchVertex {
                            pos: (&transform * vec3::from_xy(b.verts[2], 1.0)).xy(),
                            tint: RGBA32::WHITE,
                            uv: b.uvs.rd(),
                            tex_id: 0,
                            user_data: 0,
                        },
                        BatchVertex {
                            pos: (&transform * vec3::from_xy(b.verts[3], 1.0)).xy(),
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

    pub fn reset(&mut self, new_extents: vec2) {
        self.batches.clear();
        self.cursor = vec2::ZERO;
        self.extents = new_extents;
    }
}

pub struct TextBatch {
    verts: [vec2; 4], // ld, lu, ru, rd
    uvs: Rect,
    texture: TextureHandle,
}
