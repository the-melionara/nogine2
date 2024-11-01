use std::sync::Arc;

use nogine2_core::{log_warn, math::vector2::uvec2};

use crate::{colors::rgba::RGBA32, gl_wrapper::{framebuffer::GlFramebuffer, gl_clear, texture::{GlTexture, GlTextureFormat}}};

use super::{pixels::PixelFormat, Texture2D, TextureFiltering, TextureSampling, TextureWrapping};

#[derive(Debug)]
pub struct RenderTexture {
    gl_fb: GlFramebuffer,
    gl_col_att: ColAtt,

    sampling: TextureSampling,
    dims: uvec2,
}

impl RenderTexture {
    pub fn to_screen(dims: uvec2) -> Self {
        Self {
            gl_fb: GlFramebuffer::to_screen(), gl_col_att: ColAtt::Screen,
            sampling: TextureSampling { filtering: TextureFiltering::Nearest, wrapping: TextureWrapping::Clamp },
            dims,
        }
    }

    pub fn new(dims: uvec2, sampling: TextureSampling) -> Self {
        let gl_col_att = GlTexture::new(GlTextureFormat::GlR8G8B8A8, dims, sampling.filtering.into(), sampling.wrapping.into(), std::ptr::null());
        let gl_fb = GlFramebuffer::new(&gl_col_att);

        return Self { gl_fb, gl_col_att: ColAtt::Raw(gl_col_att), sampling, dims };
    }

    /// Creates a `RenderTexture` from a existing `Texture2D`. Will return `None` and throw a warning if the pixel format is not `PixelFormat::RGBA8`.
    pub fn from_texture(tex: Texture2D) -> Option<Self> {
        if tex.format != PixelFormat::RGBA8 {
            log_warn!("It is only possible to create a 'RenderTexture' from a 'Texture2D' if the texture has RGBA8 format!");
            return None;
        }

        let gl_fb = GlFramebuffer::new(&tex.gl_obj);
        return Some(Self { gl_fb, gl_col_att: ColAtt::Imported(tex.gl_obj), sampling: tex.sampling, dims: tex.dims })
    }

    /// Converts the `RenderTexture` to a `Texture2D` if the render texture is not bound to the screen. **WARNING:** The resulting texture will not have CPU access to pixel data.
    pub fn to_texture(self) -> Option<Texture2D> {
        let gl_obj = match self.gl_col_att {
            ColAtt::Screen => return None,
            ColAtt::Raw(gl_texture) => Arc::new(gl_texture),
            ColAtt::Imported(arc) => arc,
        };

        return Some(Texture2D {
            gl_obj,
            sampling: self.sampling,
            dims: self.dims,
            pixels: None,
            format: PixelFormat::RGBA8,
        })
    }

    /// Clears the colors.
    pub fn clear(&self, color: RGBA32) {
        self.bind();
        gl_clear(color);
        GlFramebuffer::to_screen().bind();
    }

    pub fn dims(&self) -> uvec2 {
        self.dims
    }

    pub(crate) fn bind(&self) {
        self.gl_fb.bind();
    }
}

#[derive(Debug)]
enum ColAtt {
    Screen,
    Raw(GlTexture),
    Imported(Arc<GlTexture>),
}
