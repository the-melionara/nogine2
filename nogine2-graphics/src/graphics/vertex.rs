use std::mem::{offset_of, size_of};

use nogine2_core::math::vector2::{bvec2, vec2};

use crate::{colors::{rgba::{RGBA32, RGBA8}, Color}, gl_wrapper::vao::{GlVertexAttribDefinition, GlVertexAttribType}};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BatchVertex {
    pub pos: vec2,
    pub tint: RGBA8,
    pub uv: vec2,

    /// Bit layout:
    /// - **4 bits:** `tex_id`
    /// - **2 bits:** `uv1`
    /// - **10 bits:** unused
    /// - **16 bits:** `user_data`
    pub ctrl: u32,
}

impl BatchVertex {
    /// Creates a new `BatchVertex`. `tint` will lose all HDR data.
    pub const fn new(
        pos: vec2,
        tint: RGBA32,
        uv: vec2,
        uv1: bvec2,
        tex_id: u32,
        user_data: u16
    ) -> Self {
        return Self {
            pos,
            tint: RGBA8::from_rgba32(tint),
            uv,
            ctrl: tex_id
                & ((uv1.0 as u32) << 4)
                & ((uv1.1 as u32) << 5)
                & ((user_data as u32) << 16)
        }
    }

    pub const fn set_tex_id(&mut self, tex_id: u32) {
        self.ctrl = self.ctrl & !0b1111 | tex_id;
    }
    
    pub(crate) const VERT_ATTRIB_DEFINITIONS: &'static [GlVertexAttribDefinition] = &[
        GlVertexAttribDefinition {
            id: 0,
            stride: size_of::<Self>(),
            offset: offset_of!(Self, pos),
            typ: GlVertexAttribType::Float,
            vec_len: 2
        },
        GlVertexAttribDefinition {
            id: 1,
            stride: size_of::<Self>(),
            offset: offset_of!(Self, tint),
            typ: GlVertexAttribType::Uint,
            vec_len: 1
        },
        GlVertexAttribDefinition {
            id: 2,
            stride: size_of::<Self>(),
            offset: offset_of!(Self, uv),
            typ: GlVertexAttribType::Float,
            vec_len: 2
        },
        GlVertexAttribDefinition {
            id: 3,
            stride: size_of::<Self>(),
            offset: offset_of!(Self, ctrl),
            typ: GlVertexAttribType::Uint,
            vec_len: 1
        },
    ];
}

impl Default for BatchVertex {
    fn default() -> Self {
        Self {
            pos: vec2::ZERO,
            tint: RGBA8::BLACK,
            uv: vec2::ZERO,
            ctrl: 0,
        }
    }
}


#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct BlitVertex {
    pub pos: vec2,
    pub tint: RGBA32,
    pub uv: vec2,
}

impl BlitVertex {
    pub(crate) const VERT_ATTRIB_DEFINITIONS: &'static [GlVertexAttribDefinition] = &[
        GlVertexAttribDefinition { id: 0, stride: size_of::<Self>(), offset: offset_of!(Self, pos ), typ: GlVertexAttribType::Float, vec_len: 2 },
        GlVertexAttribDefinition { id: 1, stride: size_of::<Self>(), offset: offset_of!(Self, tint), typ: GlVertexAttribType::Float, vec_len: 4 },
        GlVertexAttribDefinition { id: 2, stride: size_of::<Self>(), offset: offset_of!(Self, uv  ), typ: GlVertexAttribType::Float, vec_len: 2 },
    ];
}
