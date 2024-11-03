use std::sync::Arc;

use nogine2_core::{log_error, log_warn, main_thread::test_main_thread, math::{rect::IRect, vector2::{ivec2, uvec2, vec2}}};

use crate::{colors::{rgba::RGBA32, Color}, gl_wrapper::{buffer::{GlBuffer, GlBufferTarget, GlBufferUsage}, framebuffer::GlFramebuffer, gl_clear, gl_render_array, gl_uniform, gl_uniform_loc, gl_viewport, texture::{GlTexture, GlTextureFormat}, to_byte_slice, vao::GlVertexArray}, graphics::{defaults::DefaultShaders, pipeline::RenderStats, vertex::BlitVertex}};

use super::{pixels::PixelFormat, Texture2D, TextureFiltering, TextureHandle, TextureSampling, TextureWrapping};

/// A texture that can be used for rendering.
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

        return Self { gl_fb, gl_col_att: ColAtt::Offscreen(Arc::new(gl_col_att)), sampling, dims };
    }

    /// Creates a `RenderTexture` from a existing `Texture2D`. Will return `None` and throw a warning if the pixel format is not `PixelFormat::RGBA8`.
    pub fn from_texture(tex: Texture2D) -> Option<Self> {
        if tex.format != PixelFormat::RGBA8 {
            log_warn!("It is only possible to create a 'RenderTexture' from a 'Texture2D' if the texture has RGBA8 format!");
            return None;
        }

        let gl_fb = GlFramebuffer::new(&tex.gl_obj);
        return Some(Self { gl_fb, gl_col_att: ColAtt::Offscreen(tex.gl_obj), sampling: tex.sampling, dims: tex.dims })
    }

    /// Converts the `RenderTexture` to a `Texture2D` if the render texture is not bound to the screen. **WARNING:** The resulting texture will not have CPU access to pixel data.
    pub fn to_texture(self) -> Option<Texture2D> {
        let gl_obj = match self.gl_col_att {
            ColAtt::Screen => return None,
            ColAtt::Offscreen(arc) => arc,
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

    pub fn combine(&self, src: &Self, stats: &mut RenderStats) {
        self.combine_ext(src, IRect { start: ivec2::ZERO, end: ivec2::from(src.dims) }, stats);
    }

    pub fn combine_ext(&self, src: &Self, target_rect: IRect, stats: &mut RenderStats) {
        test_main_thread();

        self.bind();
        gl_viewport(target_rect);
        let vbo = GlBuffer::new(GlBufferTarget::GlArrayBuffer, to_byte_slice(&[
            BlitVertex { pos: vec2(-1.0, -1.0), tint: RGBA32::WHITE, uv: vec2(0.0, 0.0) },
            BlitVertex { pos: vec2(-1.0,  3.0), tint: RGBA32::WHITE, uv: vec2(0.0, 2.0) },
            BlitVertex { pos: vec2( 3.0, -1.0), tint: RGBA32::WHITE, uv: vec2(2.0, 0.0) },
        ]), GlBufferUsage::StaticDraw);

        let mut vao = GlVertexArray::new();
        vao.bind_vbo(&vbo, BlitVertex::VERT_ATTRIB_DEFINITIONS);

        let shader = DefaultShaders::blit();
        if !shader.use_shader() {
            log_error!("GL_ERROR: Couldn't render!");
            return;
        }

        if let Some(textures_loc) = gl_uniform_loc(shader.gl_obj(), b"uTextures\0") {
            gl_uniform::set_i32_arr(textures_loc, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
        }

        let Some(src) = src.handle() else {
            log_error!("'src' must not be a RenderTexture to the screen!");
            return;
        };

        src.bind_to(0);

        gl_render_array(3);
        GlFramebuffer::to_screen().bind();

        stats.blit.draw_calls += 1;
    }

    pub fn handle(&self) -> Option<TextureHandle> {
        match &self.gl_col_att {
            ColAtt::Screen => None,
            ColAtt::Offscreen(arc) => Some(TextureHandle { gl_obj: arc.clone() }),
        }
    }
}

#[derive(Debug)]
enum ColAtt {
    Screen,
    Offscreen(Arc<GlTexture>),
}
