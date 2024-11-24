use nogine2_core::{assert_expr, math::{rect::{IRect, Rect}, vector2::{uvec2, vec2}}};

use super::{Texture2D, TextureHandle};

/// Wrapper over `Texture2D` that simplifies access to sprites inside a sprite atlas.
#[derive(Debug, Clone)]
pub struct SpriteAtlas {
    tex: Texture2D,
    cell_size: uvec2,
}

impl SpriteAtlas {
    /// Creates a new `SpriteAtlas`. Will panic if `cell_size.0 == 0` or `cell_size.1 == 0`.
    pub fn new(texture: Texture2D, cell_size: uvec2) -> Self {
        assert_expr!(cell_size.0 > 0 && cell_size.1 > 0, "'cell_size' must be greater than 0 for every axis!");
        return Self { tex: texture, cell_size };
    }

    pub fn tex(&self) -> &Texture2D {
        &self.tex
    }

    pub fn cell_size(&self) -> uvec2 {
        self.cell_size
    }

    /// Turns `cell_size` to `1`.
    pub fn normalize(&mut self) {
        self.cell_size = uvec2::ONE;
    }

    /// Returns the same atlas with `cell_size` of `1`.
    pub fn normalized(mut self) -> Self {
        self.normalize();
        return self;
    }

    /// Samples a sprite from a `rect` in cell space. Will panic if `rect.start <= rect.end` for any axis.
    pub fn get(&self, rect: IRect) -> Sprite {
        assert_expr!(rect.start.0 < rect.end.0 && rect.start.1 < rect.end.1, "'rect.start' must be lesser than 'rect.end' for every axis!");

        let cell_count = self.tex.dims().inv_scale(self.cell_size);
        let uv_rect = Rect {
            start: vec2::from(rect.start).inv_scale(vec2::from(cell_count)),
            end: vec2::from(rect.end).inv_scale(vec2::from(cell_count)),
        };
        return Sprite { handle: self.tex.handle(), uv_rect };
    }
}


/// A segment from a `SpriteAtlas`.
#[derive(Debug, Clone)]
pub struct Sprite {
    handle: TextureHandle,
    uv_rect: Rect,
}

impl Sprite {
    pub fn handle(&self) -> &TextureHandle {
        &self.handle
    }

    pub fn uv_rect(&self) -> Rect {
        self.uv_rect
    }

    pub fn dims(&self) -> uvec2 {
        uvec2::from(vec2::from(self.handle.dims()).scale(self.uv_rect.size()))
    }
}
