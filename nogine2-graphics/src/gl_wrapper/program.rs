use std::ffi::c_char;

use nogine2_core::log_error;

use super::{gl, gl_uint, shader::GlShader};

pub struct GlProgram {
    id: gl_uint,
}

impl GlProgram {
    pub fn new(shaders: &[GlShader]) -> Option<Self> {
        unsafe {
            let id = gl::CreateProgram();
            if id == 0 {
                log_error!("Couldn't create GlProgram: Unknown error!");
                return None;
            }

            for s in shaders {
                gl::AttachShader(id, s.id());
            }

            gl::LinkProgram(id);
            let mut link_status = 0;
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut link_status);

            if link_status != gl::TRUE as i32 {
                let mut log_len = 0;
                gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut log_len);

                let mut buf = vec![0u8; log_len as usize];
                gl::GetProgramInfoLog(id, log_len, std::ptr::null_mut(), buf.as_mut_ptr() as *mut c_char);
                let msg = std::str::from_utf8_unchecked(&buf[..log_len as usize - 1]);
                log_error!("PROGRAM LINKING ERROR:\n\n{msg}");
                return None;
            }
            return Some(Self { id });
        }
    }
}
