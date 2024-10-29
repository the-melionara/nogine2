use std::ffi::c_void;

use nogine2_core::{assert_expr, crash};

use super::{gl, gl_isize, gl_uint};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlBufferTarget {
    GlArrayBuffer = gl::ARRAY_BUFFER,
    GlElementArrayBuffer = gl::ELEMENT_ARRAY_BUFFER,
    GlUniformBuffer = gl::UNIFORM_BUFFER,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlBufferUsage {
    StreamDraw = gl::STREAM_DRAW,
    StreamRead = gl::STREAM_READ,
    StreamCopy = gl::STREAM_COPY,

    StaticDraw = gl::STATIC_DRAW,
    StaticRead = gl::STATIC_READ,
    StaticCopy = gl::STATIC_COPY,

    DynamicDraw = gl::DYNAMIC_DRAW,
    DynamicRead = gl::DYNAMIC_READ,
    DynamicCopy = gl::DYNAMIC_COPY,
}

pub struct GlBuffer {
    id: gl_uint,
    target: GlBufferTarget,
    usage: GlBufferUsage,
    size: gl_isize,
}

impl GlBuffer {
    pub fn new(target: GlBufferTarget, data: &[u8], usage: GlBufferUsage) -> Self {
        let mut buf = Self::preallocated(target, data.len() as isize, usage);
        buf.set(data, 0);
        return buf;
    }

    pub fn preallocated(target: GlBufferTarget, size: gl_isize, usage: GlBufferUsage) -> Self {
        assert_expr!(size > 0);
        unsafe {
            let mut id = 0;
            gl::GenBuffers(1, &mut id);
            gl::BindBuffer(target as u32, id);
            gl::BufferData(target as u32, size, std::ptr::null(), usage as u32);

            if gl::GetError() == gl::OUT_OF_MEMORY {
                crash!("GL_ERROR: Out of memory!");
            }

            return Self { id, target, usage, size };
        }
    }

    pub fn set(&mut self, data: &[u8], offset: isize) {
        assert_expr!(offset >= 0);
        assert_expr!(data.len() as isize + offset <= self.size);

        unsafe {
            gl::BindBuffer(self.target as u32, self.id);
            gl::BufferSubData(self.target as u32, offset, data.len() as isize, data.as_ptr() as *const c_void);
        }
    }

    pub fn bind(&self) {
        unsafe { gl::BindBuffer(self.target as u32, self.id) }
    }

    pub fn target(&self) -> GlBufferTarget {
        self.target
    }
}

impl Drop for GlBuffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.id) };
    }
}
