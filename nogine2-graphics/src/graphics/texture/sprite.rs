use nogine2_core::{assert_expr, math::{rect::{IRect, Rect}, vector2::{ivec2, uvec2, vec2}}};

use super::{Texture2D, TextureHandle};

/// Wrapper over `Texture2D` that simplifies access to sprites inside a sprite atlas.
#[derive(Debug, Clone)]
pub struct SpriteAtlas {
    tex: Texture2D,
    cell_size: uvec2,
    epsilon: AtlasEpsilonMode,
}

impl SpriteAtlas {
    /// Creates a new `SpriteAtlas`. Will panic if `cell_size.0 == 0` or `cell_size.1 == 0`.
    pub fn new(texture: Texture2D, cell_size: uvec2) -> Self {
        assert_expr!(cell_size.0 > 0 && cell_size.1 > 0, "'cell_size' must be greater than 0 for every axis!");
        return Self { tex: texture, cell_size, epsilon: AtlasEpsilonMode::None };
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

    pub fn set_epsilon_mode(&mut self, epsilon: AtlasEpsilonMode) {
        self.epsilon = epsilon;
    }

    pub fn get(&self, pos: ivec2) -> Sprite {
        self.get_rect(IRect { start: pos, end: pos + ivec2::ONE })
    }

    /// Samples a sprite from a `rect` in cell space. Will panic if `rect.start <= rect.end` for any axis.
    pub fn get_rect(&self, rect: IRect) -> Sprite {
        assert_expr!(rect.start.0 < rect.end.0 && rect.start.1 < rect.end.1, "'rect.start' must be lesser than 'rect.end' for every axis!");

        let start_texel_offset = self.epsilon.start_offset();
        let end_texel_offset = self.epsilon.end_offset();
        
        let cell_count = self.tex.dims().inv_scale(self.cell_size);
        let uv_rect = Rect {
            start: (vec2::from(rect.start)
                .inv_scale(vec2::from(cell_count))
                .scale(vec2::from(self.tex.dims())) + vec2::one(start_texel_offset))
                .inv_scale(vec2::from(self.tex.dims())),
            end: (vec2::from(rect.end)
                .inv_scale(vec2::from(cell_count))
                .scale(vec2::from(self.tex.dims())) + vec2::one(end_texel_offset))
                .inv_scale(vec2::from(self.tex.dims())),
        };
        let dims = self.cell_size.scale(uvec2::from(rect.size()));
        return Sprite { handle: self.tex.handle(), uv_rect, dims };
    }
}


/// A segment from a `SpriteAtlas`.
#[derive(Debug, Clone)]
pub struct Sprite {
    handle: TextureHandle,
    uv_rect: Rect,
    dims: uvec2,
}

impl Sprite {
    pub fn handle(&self) -> &TextureHandle {
        &self.handle
    }

    pub fn uv_rect(&self) -> Rect {
        self.uv_rect
    }

    pub fn dims(&self) -> uvec2 {
        self.dims
    }
}

/// Defines how the atlas will add margins and shift the UVs to avoid visual artifacts.
#[derive(Debug, Clone, Copy)]
pub enum AtlasEpsilonMode {
    None,
    /// Fit for pixel perfect rendering.
    PixelPerfect,
    /// Fir for not pixel perfect rendering where tiles bleed into each other. `texels` is measured in texture pixels.
    Shrinking { texels: f32 },
}

impl AtlasEpsilonMode {
    const PIXEL_PERFECT_TEXEL: f32 = 0.1;
    
    fn start_offset(&self) -> f32 {
        match self {
            AtlasEpsilonMode::None => 0.0,
            AtlasEpsilonMode::PixelPerfect => Self::PIXEL_PERFECT_TEXEL,
            AtlasEpsilonMode::Shrinking { texels } => *texels,
        }
    }

    fn end_offset(&self) -> f32 {
        match self {
            AtlasEpsilonMode::None => 0.0,
            AtlasEpsilonMode::PixelPerfect => Self::PIXEL_PERFECT_TEXEL,
            AtlasEpsilonMode::Shrinking { texels } => -(*texels),
        }
    }
}
