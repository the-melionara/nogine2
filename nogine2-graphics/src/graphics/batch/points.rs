use std::sync::Arc;

use nogine2_core::{log_error, math::mat3x3::mat3};

use crate::{gl_wrapper::{buffer::{GlBuffer, GlBufferTarget, GlBufferUsage}, gl_render_array, gl_uniform, to_byte_slice, vao::GlVertexArray, GlRenderMode}, graphics::{blending::BlendingMode, material::Material, vertex::BatchVertex}};

pub struct PtsBatchRenderCall {
    buffers: PtsBatchBuffers,
    blending: BlendingMode,
    material: Arc<Material>,
}

impl PtsBatchRenderCall { 
    pub fn new(buffers: PtsBatchBuffers, blending: BlendingMode, material: Arc<Material>) -> Self {
        Self { buffers, blending, material }
    }

    pub fn render(&self, view_mat: &mat3) {
        let verts_len = self.buffers.bind_all();

        if !self.material.use_material() {
            log_error!("GL_ERROR: Couldn't render!");
            return;
        }
    
        if let Some(view_mat_loc) = self.material.uniform_loc(c"uViewMat") {
            gl_uniform::set_mat3(view_mat_loc, view_mat);
        }
   
        self.blending.apply();

        gl_render_array(GlRenderMode::GlPoints, verts_len);
    }

    /// Clears the buffers first
    pub fn recycle(mut self) -> PtsBatchBuffers {
        self.buffers.clear();
        self.buffers
    }

    pub fn allows(&self, verts_len: usize, blending: BlendingMode, material: &Arc<Material>) -> bool {
        self.buffers.fits(verts_len) && self.blending == blending && *self.material == **material
    }

    pub fn push(&mut self, verts: &mut [BatchVertex]) {
        for v in &mut *verts {
            v.tex_id = 0;
        }

        self.buffers.push(verts);
    }

    pub fn on_use_size(&self) -> usize {
        self.buffers.on_use_size()
    }
}


pub struct PtsBatchBuffers {
    vbo: GlBuffer,
    vlen: usize,
    vao: GlVertexArray,
}

impl PtsBatchBuffers {
    pub const MAX_PTS: usize = 64;
    pub const BYTE_SIZE: usize = Self::MAX_PTS * size_of::<BatchVertex>();

    pub fn new() -> Self {
        let mut item = Self {
            vbo: GlBuffer::preallocated(GlBufferTarget::GlArrayBuffer, (Self::MAX_PTS * size_of::<BatchVertex>()) as isize, GlBufferUsage::DynamicDraw),
            vao: GlVertexArray::new(),
            vlen: 0,
        };
        item.vao.bind_vbo(&item.vbo, BatchVertex::VERT_ATTRIB_DEFINITIONS);
        return item;
    }

    fn on_use_size(&self) -> usize {
        self.vlen * size_of::<BatchVertex>()
    }

    fn fits(&self, verts: usize) -> bool {
        return self.vlen + verts <= Self::MAX_PTS;
    }

    fn push(&mut self, verts: &[BatchVertex]) {
        if !self.fits(verts.len()) {
            return;
        }

        self.vbo.set(to_byte_slice(verts), (self.vlen * size_of::<BatchVertex>()) as isize);
        self.vlen += verts.len();
    }

    /// Returns the verts count.
    fn bind_all(&self) -> i32 {
        self.vao.bind();
        return self.vlen as i32;
    }

    fn clear(&mut self) {
        self.vlen = 0;
    }
}
