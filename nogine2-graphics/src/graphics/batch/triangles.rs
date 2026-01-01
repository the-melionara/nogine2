use std::sync::Arc;

use nogine2_core::{log_error, math::mat3x3::mat3};

use crate::{gl_wrapper::{buffer::{GlBuffer, GlBufferTarget, GlBufferUsage}, gl_render_elements, gl_uniform, to_byte_slice, vao::GlVertexArray, GlRenderMode}, graphics::{blending::BlendingMode, material::Material, texture::TextureHandle, vertex::BatchVertex}};

pub struct TriBatchRenderCall {
    buffers: TriBatchBuffers,
    textures: Vec<TextureHandle>,
    blending: BlendingMode,
    material: Arc<Material>,
    tex_offset: usize,
}

impl TriBatchRenderCall {
    const MAX_TEXTURES: usize = 16;
    const TEXTURES: [i32; Self::MAX_TEXTURES] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    
    pub fn new(buffers: TriBatchBuffers, blending: BlendingMode, material: Arc<Material>) -> Self {
        Self {
            buffers,
            textures: Vec::new(),
            blending,
            tex_offset: material.sampler_count(),
            material,
        }
    }

    pub fn render(&self, view_mat: &mat3) {
        let indices_len = self.buffers.bind_all();

        if !self.material.use_material() {
            log_error!("GL_ERROR: Couldn't render!");
            return;
        }

        let sampler_count = self.material.sampler_count();
        for (i, t) in self.textures.iter().enumerate() {
            t.bind_to((i + sampler_count) as u32); // offseted to avoid uniform samplers
        }
    
        if let Some(view_mat_loc) = self.material.uniform_loc(c"uViewMat") {
            gl_uniform::set_mat3(view_mat_loc, view_mat);
        }

        if let Some(textures_loc) = self.material.uniform_loc(c"uTextures") {
            gl_uniform::set_i32_arr(textures_loc, &Self::TEXTURES);
        }

        self.blending.apply();

        gl_render_elements(GlRenderMode::GlTriangles, indices_len);
    }

    /// Clears the buffers first
    pub fn recycle(mut self) -> TriBatchBuffers {
        self.buffers.clear();
        self.buffers
    }

    pub fn allows(&self, verts_len: usize, indices_len: usize, texture: &TextureHandle, blending: BlendingMode, material: &Arc<Material>) -> bool {
        self.buffers.fits(verts_len, indices_len)
            && (
                self.textures.len() + self.tex_offset < Self::MAX_TEXTURES
                    || self.textures.contains(texture)
            )
            && self.blending == blending
            && *self.material == **material
    }

    pub fn push(&mut self, verts: &mut [BatchVertex], indices: &mut [u16], texture: TextureHandle) {
        let tex_id = match self.textures.iter().position(|t| t == &texture) {
            Some(i) => i as u32,
            None => {
                self.textures.push(texture); // WARN: Possible memory allocation
                (self.textures.len() - 1) as u32
            },
        } + self.tex_offset as u32;

        for v in &mut *verts {
            v.tex_id = tex_id;
        }

        self.buffers.push(verts, indices);
    }

    pub fn on_use_size(&self) -> usize {
        self.buffers.on_use_size()
    }
}


pub struct TriBatchBuffers {
    vbo: GlBuffer,
    ebo: GlBuffer,
    vlen: usize,
    elen: usize,
    vao: GlVertexArray,
}

impl TriBatchBuffers {
    const MAX_QUADS: usize = 2048;
    pub const MAX_VERTS: usize = Self::MAX_QUADS * 4;
    pub const MAX_INDICES: usize = Self::MAX_QUADS * 6;
    pub const BYTE_SIZE: usize = Self::MAX_VERTS * size_of::<BatchVertex>() + Self::MAX_INDICES * size_of::<u16>();

    pub fn new() -> Self {
        let mut item = Self {
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

    fn push(&mut self, verts: &[BatchVertex], indices: &mut [u16]) {
        if !self.fits(verts.len(), indices.len()) {
            return;
        }

        for i in &mut *indices {
            *i += self.vlen as u16;
        }
        self.vbo.set(to_byte_slice(verts), (self.vlen * size_of::<BatchVertex>()) as isize);
        self.ebo.set(to_byte_slice(indices), (self.elen * size_of::<u16>()) as isize);

        self.vlen += verts.len();
        self.elen += indices.len();
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
