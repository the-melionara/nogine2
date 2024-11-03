use nogine2_core::main_thread::test_main_thread;

use super::{gl, gl_uint, texture::GlTexture};

#[derive(Debug)]
pub struct GlFramebuffer {
    id: gl_uint
}

impl GlFramebuffer {
    pub fn to_screen() -> Self {
        Self { id: 0 }
    }

    pub fn new(color_att: &GlTexture) -> Self {
        test_main_thread();
        unsafe {
            let mut id = 0;
            gl::GenFramebuffers(1, &mut id);

            gl::BindFramebuffer(gl::FRAMEBUFFER, id);
            gl::FramebufferTexture(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, color_att.id(), 0);
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

            return Self { id };
        }
    }

    pub fn bind(&self) {
        test_main_thread();
        unsafe { gl::BindFramebuffer(gl::FRAMEBUFFER, self.id) };
    }
}

impl Drop for GlFramebuffer {
    fn drop(&mut self) {
        if self.id != 0 {
            test_main_thread();
            unsafe { gl::DeleteFramebuffers(1, &self.id) };
        }
    }
}
