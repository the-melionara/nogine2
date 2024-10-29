use std::ffi::c_void;

use crate::colors::rgba::RGBA32;

pub mod buffer;
pub mod vao;
pub mod shader;
pub mod program;

mod gl;

#[allow(non_camel_case_types)] pub type gl_uint = gl::types::GLuint;
#[allow(non_camel_case_types)] pub type gl_isize = gl::types::GLsizeiptr;

pub fn gl_load(f: impl Fn(&str) -> *const c_void) {
    gl::load_with(f);
}

pub fn gl_enable_blend() {
    unsafe { gl::Enable(gl::BLEND) };
}

pub fn gl_clear(col: RGBA32) {
    unsafe {
        gl::ClearColor(col.0, col.1, col.2, col.3);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}
