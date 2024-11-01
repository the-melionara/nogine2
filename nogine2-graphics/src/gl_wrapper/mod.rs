use std::ffi::{c_char, c_void};

use nogine2_core::{assert_expr, main_thread::test_main_thread, math::rect::IRect};
use program::GlProgram;

use crate::colors::rgba::RGBA32;

pub mod buffer;
pub mod vao;
pub mod shader;
pub mod program;
pub mod texture;

mod gl;

#[allow(non_camel_case_types)] pub type gl_uint = gl::types::GLuint;
#[allow(non_camel_case_types)] pub type gl_isize = gl::types::GLsizeiptr;

pub fn gl_load(f: impl Fn(&str) -> *const c_void) {
    gl::load_with(f);
}

pub fn gl_enable_blend() {
    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }
}

pub fn gl_clear(col: RGBA32) {
    //test_main_thread(); // not needed
    unsafe {
        gl::ClearColor(col.0, col.1, col.2, col.3);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}

pub fn gl_render_elements(indices_count: i32) {
    //test_main_thread(); // not needed
    assert_expr!(indices_count >= 0);
    unsafe {
        gl::DrawElements(gl::TRIANGLES, indices_count, gl::UNSIGNED_SHORT, std::ptr::null());
    }
}

pub fn gl_viewport(rect: IRect) {
    //test_main_thread(); // not needed
    assert_expr!(rect.start.0 < rect.end.0 && rect.start.1 < rect.end.1);
    unsafe {
        gl::Viewport(rect.start.0, rect.start.1, rect.size().0, rect.size().1);
    }
}

pub fn gl_uniform_loc(program: &GlProgram, name: &[u8]) -> Option<i32> {
    test_main_thread();
    assert_expr!(name.last().copied() == Some(b'\0'));
    unsafe {
        let i = gl::GetUniformLocation(program.id(), name.as_ptr() as *const c_char);
        if i < 0 {
            return None;
        } else {
            return Some(i);
        }
    }
}

pub mod gl_uniform {
    use nogine2_core::math::mat3x3::mat3;

    use super::gl;

    pub fn set_mat3(loc: i32, mat: &mat3) {
        //test_main_thread(); // not needed
        unsafe { gl::UniformMatrix3fv(loc, 1, gl::TRUE, mat.ptr()) };
    }

    pub fn set_i32_arr(loc: i32, arr: &[i32]) {
        //test_main_thread(); // not needed
        unsafe { gl::Uniform1iv(loc, arr.len() as i32, arr.as_ptr()) };
    }
}

pub fn to_byte_slice<T>(slice: &[T]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(slice.as_ptr() as *const u8, slice.len() * std::mem::size_of::<T>()) }
}
