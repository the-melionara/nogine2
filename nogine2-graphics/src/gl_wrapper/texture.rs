use std::ffi::c_void;

use nogine2_core::{assert_expr, main_thread::test_main_thread, math::vector2::uvec2};

use super::{gl, gl_uint};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlTextureFiltering {
    GlNearest = gl::NEAREST,
    GlLinear = gl::LINEAR,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlTextureWrapping {
    GlClamp = gl::CLAMP_TO_EDGE,
    GlRepeat = gl::REPEAT,
    GlMirroredRepeat = gl::MIRRORED_REPEAT,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlTextureFormat {
    GlR8 = gl::RED,
    GlR8G8 = gl::RG,
    GlR8G8B8A8 = gl::RGBA,
}

#[derive(Debug)]
pub struct GlTexture {
    id: gl_uint,
    dims: uvec2,
}

impl GlTexture {
    pub fn new(format: GlTextureFormat, dims: uvec2, filtering: GlTextureFiltering, wrapping: GlTextureWrapping, data: *const c_void) -> Self {
        test_main_thread();
        assert_expr!(dims.0 > 0 && dims.0 < gl::MAX_TEXTURE_SIZE && dims.1 > 0 && dims.1 < gl::MAX_TEXTURE_SIZE);

        unsafe {
            let mut id = 0;
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, filtering as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, filtering as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, wrapping as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, wrapping as i32);

            gl::TexImage2D(gl::TEXTURE_2D, 0, format as i32, dims.0 as i32, dims.1 as i32, 0, format as u32, gl::UNSIGNED_BYTE, data);
            return Self { id, dims };
        }
    }

    pub fn set(&self, offset: uvec2, dims: uvec2, format: GlTextureFormat, data: *const c_void) {
        test_main_thread();
        assert_expr!(offset.0 + dims.0 <= self.dims.0 && offset.1 + dims.1 <= self.dims.1);

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexSubImage2D(gl::TEXTURE_2D, 0, offset.0 as i32, offset.1 as i32, dims.0 as i32, dims.1 as i32, format as u32, gl::UNSIGNED_BYTE, data);
        }
    }

    pub fn bind_to(&self, target: u32) {
        test_main_thread();
        assert_expr!(target < gl::MAX_TEXTURE_IMAGE_UNITS);

        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + target);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}

impl PartialEq for GlTexture {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for GlTexture {}

impl Drop for GlTexture {
    fn drop(&mut self) {
        test_main_thread();
        unsafe { gl::DeleteTextures(1, &self.id) };
    }
}
