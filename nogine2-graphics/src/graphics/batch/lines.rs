use std::sync::Arc;

use nogine2_core::{log_error, math::{mat3x3::mat3, vector2::vec2}};

use crate::{gl_wrapper::{buffer::{GlBuffer, GlBufferTarget, GlBufferUsage}, gl_render_elements, gl_uniform, to_byte_slice, vao::GlVertexArray, GlRenderMode}, graphics::{blending::BlendingMode, material::Material, vertex::BatchVertex}};

pub struct LnsBatchRenderCall {
    buffers: LnsBatchBuffers,
    blending: BlendingMode,
    material: Arc<Material>,
}

impl LnsBatchRenderCall {
    pub fn new(buffers: LnsBatchBuffers, blending: BlendingMode, material: Arc<Material>) -> Self {
        Self { buffers, blending, material }
    }

    pub fn render(&self, view_mat: &mat3) {
        let indices_len = self.buffers.bind_all();

        if !self.material.use_material() {
            log_error!("GL_ERROR: Couldn't render!");
            return;
        }
    
        if let Some(view_mat_loc) = self.material.uniform_loc(b"uViewMat\0") {
            gl_uniform::set_mat3(view_mat_loc, view_mat);
        }

        self.blending.apply();

        gl_render_elements(GlRenderMode::GlLines, indices_len);
    }

    /// Clears the buffers first
    pub fn recycle(mut self) -> LnsBatchBuffers {
        self.buffers.clear();
        self.buffers
    }

    pub fn allows(&self, verts_len: usize, indices_len: usize, blending: BlendingMode, material: &Arc<Material>) -> bool {
        self.buffers.fits(verts_len, indices_len) && self.blending == blending && *self.material == **material
    }

    pub fn push(&mut self, mut verts: [BatchVertex; 2]) {
        for v in &mut verts {
            v.tex_id = 0;
            v.uv = vec2::ZERO;
        }

        self.buffers.push(verts);
    }

    pub fn on_use_size(&self) -> usize {
        self.buffers.on_use_size()
    }
}


pub struct LnsBatchBuffers {
    verts: Vec<BatchVertex>,
    vbo: GlBuffer,
    ebo: GlBuffer,
    vlen: usize,
    elen: usize,
    vao: GlVertexArray,
}

impl LnsBatchBuffers {
    const MAX_LINES: usize = 64;
    pub const MAX_VERTS: usize = Self::MAX_LINES * 2;
    pub const MAX_INDICES: usize = Self::MAX_LINES * 2;
    pub const BYTE_SIZE: usize = Self::MAX_VERTS * size_of::<BatchVertex>() + Self::MAX_INDICES * size_of::<u16>();

    pub fn new() -> Self {
        let mut item = Self {
            verts: Vec::new(),
            vbo: GlBuffer::preallocated(GlBufferTarget::GlArrayBuffer, (Self::MAX_VERTS * size_of::<BatchVertex>()) as isize, GlBufferUsage::DynamicDraw),
            ebo: GlBuffer::preallocated(GlBufferTarget::GlElementArrayBuffer, (Self::MAX_INDICES * size_of::<u16>()) as isize, GlBufferUsage::DynamicDraw),
            vao: GlVertexArray::new(),
            vlen: 0, elen: 0,
        };
        item.vao.bind_vbo(&item.vbo, BatchVertex::VERT_ATTRIB_DEFINITIONS);
        return item;
    }

    fn on_use_size(&self) -> usize {
        self.vlen * size_of::<BatchVertex>() + self.elen * size_of::<u16>()
    }

    fn fits(&self, verts: usize, indices: usize) -> bool {
        return self.vlen + verts <= Self::MAX_VERTS && self.elen + indices <= Self::MAX_INDICES;
    }

    fn push(&mut self, verts: [BatchVertex; 2]) {
        if !self.fits(verts.len(), 2) {
            return;
        }

        let first_index = match self.verts.iter().position(|x| x == &verts[0]) {
            Some(i) => i,
            None => {
                self.verts.push(verts[0]);
                self.vbo.set(to_byte_slice(&[verts[0]]), (self.vlen * size_of::<BatchVertex>()) as isize);
                self.vlen += 1;
                self.verts.len() - 1
            },
        } as u16;

        let second_index = match self.verts.iter().position(|x| x == &verts[1]) {
            Some(i) => i,
            None => {
                self.verts.push(verts[1]);
                self.vbo.set(to_byte_slice(&[verts[1]]), (self.vlen * size_of::<BatchVertex>()) as isize);
                self.vlen += 1;
                self.verts.len() - 1
            },
        } as u16;

        self.ebo.set(to_byte_slice(&[first_index, second_index]), (self.elen * size_of::<u16>()) as isize);
        self.elen += 2;
    }

    /// Returns the indices count.
    fn bind_all(&self) -> i32 {
        self.vao.bind();
        self.ebo.bind();
        return self.elen as i32;
    }

    fn clear(&mut self) {
        self.vlen = 0;
        self.elen = 0;
    }
}
