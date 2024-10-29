use std::ffi::c_char;

use nogine2_core::log_error;

use super::{gl, gl_uint};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlShaderType {
    GlVertexShader = gl::VERTEX_SHADER,
    GlFragmentShader = gl::FRAGMENT_SHADER,
    GlGeometryShader = gl::GEOMETRY_SHADER,
}

#[derive(Debug)]
pub struct GlShader {
    id: gl_uint,
    typ: GlShaderType,
}

impl GlShader {
    /// `src` is not zero terminated
    pub fn new(typ: GlShaderType, src: &[u8]) -> Option<Self> {
        let (len, ptr) = (src.len() as i32, src.as_ptr() as *const c_char);
        unsafe {
            let id = gl::CreateShader(typ as u32);
            gl::ShaderSource(id, 1, &ptr, &len);
            gl::CompileShader(id);

            let mut compile_status = 0;
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut compile_status);

            if compile_status != gl::TRUE as i32 {
                let mut log_len = 0;
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut log_len);

                let mut buf = vec![0u8; log_len as usize];
                gl::GetShaderInfoLog(id, log_len, std::ptr::null_mut(), buf.as_mut_ptr() as *mut c_char);
                let msg = std::str::from_utf8_unchecked(&buf[..log_len as usize - 1]);
                log_error!("{} SHADER COMPILATION ERROR:\n\n{msg}", match typ {
                    GlShaderType::GlVertexShader => "VERTEX",
                    GlShaderType::GlFragmentShader => "FRAGMENT",
                    GlShaderType::GlGeometryShader => "GEOMETRY",
                });
                return None;
            }

            return Some(Self { id, typ });
        }
    }

    pub fn typ(&self) -> GlShaderType {
        self.typ
    }

    pub(super) fn id(&self) -> gl_uint {
        self.id
    }
}

impl PartialEq for GlShader {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for GlShader { }

impl Drop for GlShader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.id) };
    }
}
