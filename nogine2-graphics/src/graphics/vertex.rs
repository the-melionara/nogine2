use std::mem::{offset_of, size_of};

use nogine2_core::math::vector2::vec2;

use crate::{colors::rgba::RGBA32, gl_wrapper::vao::{GlVertexAttribDefinition, GlVertexAttribType}};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BatchVertex {
    pub pos: vec2,
    pub tint: RGBA32,
    pub uv: vec2,
    pub tex_id: u32,
    pub user_data: i32,
}

impl BatchVertex {
    pub(crate) const VERT_ATTRIB_DEFINITIONS: &'static [GlVertexAttribDefinition] = &[
        GlVertexAttribDefinition { id: 0, stride: size_of::<Self>(), offset: offset_of!(Self, pos      ), typ: GlVertexAttribType::Float, vec_len: 2 },
        GlVertexAttribDefinition { id: 1, stride: size_of::<Self>(), offset: offset_of!(Self, tint     ), typ: GlVertexAttribType::Float, vec_len: 4 },
        GlVertexAttribDefinition { id: 2, stride: size_of::<Self>(), offset: offset_of!(Self, uv       ), typ: GlVertexAttribType::Float, vec_len: 2 },
        GlVertexAttribDefinition { id: 3, stride: size_of::<Self>(), offset: offset_of!(Self, tex_id   ), typ: GlVertexAttribType::Uint,  vec_len: 1 },
        GlVertexAttribDefinition { id: 4, stride: size_of::<Self>(), offset: offset_of!(Self, user_data), typ: GlVertexAttribType::Int,   vec_len: 1 },
    ];
}
