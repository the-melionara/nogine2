use std::ffi::{c_char, c_void};

use nogine2_core::{assert_expr, main_thread::test_main_thread, math::rect::IRect};
use program::GlProgram;

use crate::colors::rgba::RGBA32;

pub mod buffer;
pub mod vao;
pub mod shader;
pub mod program;
pub mod texture;
pub mod framebuffer;

mod gl;

#[allow(non_camel_case_types)] pub type gl_uint = gl::types::GLuint;
#[allow(non_camel_case_types)] pub type gl_isize = gl::types::GLsizeiptr;

pub fn gl_load(f: impl Fn(&str) -> *const c_void) {
    gl::load_with(f);
}

pub fn gl_enable_blend() {
    unsafe { gl::Enable(gl::BLEND) };
}

pub fn gl_alpha_blend() {
    unsafe {
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::BlendEquation(gl::FUNC_ADD);
    }
}

pub fn gl_additive_blend() {
    unsafe {
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE);
        gl::BlendEquation(gl::FUNC_ADD);
    }
}

pub fn gl_subtractive_blend() {
    unsafe {
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE);
        gl::BlendEquation(gl::FUNC_REVERSE_SUBTRACT);
    }
}

pub fn gl_multiplicative_blend() {
    unsafe {
        gl::BlendFunc(gl::DST_COLOR, gl::ZERO);
        gl::BlendEquation(gl::FUNC_ADD);
    }
}

pub fn gl_clear(col: RGBA32) {
    //test_main_thread(); // not needed
    unsafe {
        gl::ClearColor(col.0, col.1, col.2, col.3);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}


#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum GlRenderMode {
    GlTriangles = gl::TRIANGLES,
    GlLines = gl::LINES,
    GlPoints = gl::POINTS,
}

pub fn gl_render_elements(mode: GlRenderMode, indices_count: i32) {
    //test_main_thread(); // not needed
    assert_expr!(indices_count >= 0);
    unsafe {
        gl::DrawElements(mode as u32, indices_count, gl::UNSIGNED_SHORT, std::ptr::null());
    }
}

pub fn gl_render_array(mode: GlRenderMode, verts_count: i32) {
    assert_expr!(verts_count >= 0);
    unsafe {
        gl::DrawArrays(mode as u32, 0, verts_count);
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
    use nogine2_core::math::{mat3x3::mat3, vector2::{ivec2, uvec2, vec2}, vector3::{ivec3, uvec3, vec3}, vector4::{ivec4, uvec4, vec4}};

    use super::gl;

    pub fn set_i32(loc: i32, val: i32) { unsafe { gl::Uniform1i(loc, val) } }
    pub fn set_ivec2(loc: i32, val: ivec2) { unsafe { gl::Uniform2i(loc, val.0, val.1) } }
    pub fn set_ivec3(loc: i32, val: ivec3) { unsafe { gl::Uniform3i(loc, val.0, val.1, val.2) } }
    pub fn set_ivec4(loc: i32, val: ivec4) { unsafe { gl::Uniform4i(loc, val.0, val.1, val.2, val.3) } }

    pub fn set_u32(loc: i32, val: u32) { unsafe { gl::Uniform1ui(loc, val) } }
    pub fn set_uvec2(loc: i32, val: uvec2) { unsafe { gl::Uniform2ui(loc, val.0, val.1) } }
    pub fn set_uvec3(loc: i32, val: uvec3) { unsafe { gl::Uniform3ui(loc, val.0, val.1, val.2) } }
    pub fn set_uvec4(loc: i32, val: uvec4) { unsafe { gl::Uniform4ui(loc, val.0, val.1, val.2, val.3) } }

    pub fn set_f32(loc: i32, val: f32) { unsafe { gl::Uniform1f(loc, val) } }
    pub fn set_vec2(loc: i32, val: vec2) { unsafe { gl::Uniform2f(loc, val.0, val.1) } }
    pub fn set_vec3(loc: i32, val: vec3) { unsafe { gl::Uniform3f(loc, val.0, val.1, val.2) } }
    pub fn set_vec4(loc: i32, val: vec4) { unsafe { gl::Uniform4f(loc, val.0, val.1, val.2, val.3) } }

    pub fn set_mat3(loc: i32, mat: &mat3) {
        unsafe { gl::UniformMatrix3fv(loc, 1, gl::TRUE, mat.ptr()) };
    }

    pub fn set_i32_arr(loc: i32, arr: &[i32]) {
        unsafe { gl::Uniform1iv(loc, arr.len() as i32, arr.as_ptr()) };
    }

    pub fn uniforms_failed() -> bool {
        return unsafe { gl::GetError() == gl::INVALID_OPERATION };
    }
}

pub fn to_byte_slice<T>(slice: &[T]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(slice.as_ptr() as *const u8, slice.len() * std::mem::size_of::<T>()) }
}
