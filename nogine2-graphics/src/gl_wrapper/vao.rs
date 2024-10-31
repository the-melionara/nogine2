use std::ffi::c_void;

use nogine2_core::assert_expr;

use crate::gl_wrapper::buffer::GlBufferTarget;

use super::{buffer::GlBuffer, gl, gl_uint};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GlVertexAttribDefinition {
    pub id: u32,
    pub stride: usize,
    pub offset: usize,
    pub typ: GlVertexAttribType,
    pub vec_len: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlVertexAttribType {
    Float, Int, Uint
}

// !!!!!! test_main_thread() is not needed here because the user will never make this manually or
// anything of the sorts. This is only used by utils that only exist on the main therad.

pub struct GlVertexArray {
    id: gl_uint,
}

impl GlVertexArray {
    pub fn new() -> Self {
        unsafe {
            let mut id = 0;
            gl::GenVertexArrays(1, &mut id);
            return Self { id };
        }
    }

    pub fn bind_vbo(&mut self, buffer: &GlBuffer, def: &[GlVertexAttribDefinition]) {
        assert_expr!(buffer.target() == GlBufferTarget::GlArrayBuffer);
        unsafe {
            gl::BindVertexArray(self.id);
            buffer.bind();

            for att in def {
                assert_expr!(att.id < gl::MAX_VERTEX_ATTRIBS);
                assert_expr!(matches!(att.vec_len, 1 | 2 | 3 | 4));

                gl::EnableVertexAttribArray(att.id);
                match att.typ {
                    GlVertexAttribType::Float => gl::VertexAttribPointer(att.id, att.vec_len as i32, gl::FLOAT, gl::FALSE, att.stride as i32, att.offset as *const c_void),
                    GlVertexAttribType::Int => gl::VertexAttribIPointer(att.id, att.vec_len as i32, gl::INT, att.stride as i32, att.offset as *const c_void),
                    GlVertexAttribType::Uint => gl::VertexAttribIPointer(att.id, att.vec_len as i32, gl::UNSIGNED_INT, att.stride as i32, att.offset as *const c_void),
                }
            }
        }
    }

    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.id) };
    }
}

impl Drop for GlVertexArray {
    fn drop(&mut self) {
        unsafe { gl::DeleteVertexArrays(1, &self.id) };
    }
}
