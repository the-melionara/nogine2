use std::mem::size_of;

use nogine2_core::{log_error, math::{mat3x3::mat3, rect::Rect, vector2::{uvec2, vec2}}};

use crate::gl_wrapper::{buffer::{GlBuffer, GlBufferTarget, GlBufferUsage}, gl_render_elements, gl_uniform, gl_uniform_loc, to_byte_slice, vao::GlVertexArray};

use super::{defaults::DefaultShaders, pipeline::BatchRenderStats, vertex::BatchVertex, CameraData};

pub struct BatchData {
    render_calls: Vec<BatchRenderCall>,
    pooled_buffers: Vec<BatchBuffers>,

    view_mat: mat3,
    cam_rect: Rect,
    snapping: vec2,
    camera: CameraData,

    stats: BatchRenderStats,
}

impl BatchData {
    pub const fn new() -> Self {
        Self {
            render_calls: Vec::new(), pooled_buffers: Vec::new(),
            view_mat: mat3::IDENTITY, cam_rect: Rect::IDENT, snapping: vec2::ONE, camera: CameraData { center: vec2::ZERO, extents: vec2::ZERO },
            stats: BatchRenderStats::new(),
        }
    }

    pub fn push(&mut self, cmd: BatchPushCmd<'_>) {
        let bb = calculate_bounding_box(cmd.verts);
        if !aabb_check(self.cam_rect, bb) {
            self.stats.skipped_submissions += 1;
            return;
        }
        self.stats.rendered_submissions += 1;

        let verts = cmd.verts.iter().copied().map(|mut x| {
            x.pos = snap(x.pos, self.snapping);
            return x;
        }).collect::<Vec<_>>();
        let mut indices = cmd.indices.to_vec();

        self.stats.verts += verts.len();
        self.stats.triangles += indices.len() / 3;

        let cursor = self.render_call_cursor(verts.len(), indices.len());
        self.render_calls[cursor].push(&verts, &mut indices);
    }

    pub fn setup_frame(&mut self, mut camera: CameraData, target_res: uvec2) {
        let snapping = vec2::from(target_res).inv_scale(camera.extents);
        camera.center = snap(camera.center, snapping);

        self.snapping = snapping;
        self.view_mat = mat3::tf_matrix(camera.center, 0.0, camera.extents.scale(vec2(1.0, -1.0) * 0.5)).inverse().unwrap_or(mat3::IDENTITY);
        self.cam_rect = Rect { start: camera.center - camera.extents * 0.5, end: camera.center + camera.extents * 0.5 };
        self.camera = camera;
        self.stats = BatchRenderStats::new();

        self.clear();
    }

    fn clear(&mut self) {
        self.pooled_buffers.clear();
        while let Some(call) = self.render_calls.pop() {
            self.pooled_buffers.push(call.recycle());
        }
    }

    pub fn render(&mut self) -> BatchRenderStats {
        for call in &self.render_calls {
            call.render(&self.view_mat);
            self.stats.draw_calls += 1;
        }
        return self.stats.clone();
    } 

    pub fn camera(&self) -> CameraData {
        self.camera.clone()
    }

    fn render_call_cursor(&mut self, verts_len: usize, indices_len: usize) -> usize {
        if let Some(last) = self.render_calls.last() {
            if last.allows(verts_len, indices_len) {
                return self.render_calls.len() - 1;
            }
        }
        let buffers = self.make_or_fetch_buffers();
        self.render_calls.push(BatchRenderCall::new(buffers));
        return self.render_calls.len() - 1;
    }

    fn make_or_fetch_buffers(&mut self) -> BatchBuffers {
        match self.pooled_buffers.pop() {
            Some(x) => x,
            None => BatchBuffers::new(),
        }
    }
}

pub struct BatchPushCmd<'a> {
    pub verts: &'a [BatchVertex],
    pub indices: &'a [u16],
}


struct BatchRenderCall {
    buffers: BatchBuffers,
}

impl BatchRenderCall {
    fn new(buffers: BatchBuffers) -> Self {
        Self { buffers }
    }

    fn render(&self, view_mat: &mat3) {
        let indices_len = self.buffers.bind_all();

        let shader = DefaultShaders::batch();
        if !shader.use_shader() {
            log_error!("GL_ERROR: Couldn't render!");
            return;
        }
    
        if let Some(view_mat_loc) = gl_uniform_loc(shader.gl_obj(), b"uViewMat\0") {
            gl_uniform::set_mat3(view_mat_loc, view_mat);
        }
    
        gl_render_elements(indices_len);
    }

    /// Clears the buffers first
    fn recycle(mut self) -> BatchBuffers {
        self.buffers.clear();
        self.buffers
    }

    fn allows(&self, verts_len: usize, indices_len: usize) -> bool {
        self.buffers.fits(verts_len, indices_len)
    }

    fn push(&mut self, verts: &[BatchVertex], indices: &mut [u16]) {
        self.buffers.push(verts, indices);
    }
}


struct BatchBuffers {
    vbo: GlBuffer,
    ebo: GlBuffer,
    vlen: usize,
    elen: usize,
    vao: GlVertexArray,
}

impl BatchBuffers {
    const MAX_QUADS: usize = 256;
    const MAX_VERTS: usize = Self::MAX_QUADS * 4;
    const MAX_INDICES: usize = Self::MAX_QUADS * 6;

    fn new() -> Self {
        let mut item = Self {
            vbo: GlBuffer::preallocated(GlBufferTarget::GlArrayBuffer, (Self::MAX_VERTS * size_of::<BatchVertex>()) as isize, GlBufferUsage::DynamicDraw),
            ebo: GlBuffer::preallocated(GlBufferTarget::GlElementArrayBuffer, (Self::MAX_INDICES * size_of::<u16>()) as isize, GlBufferUsage::DynamicDraw),
            vao: GlVertexArray::new(),
            vlen: 0, elen: 0,
        };
        item.vao.bind_vbo(&item.vbo, BatchVertex::VERT_ATTRIB_DEFINITIONS);
        return item;
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

fn snap(x: vec2, snapping: vec2) -> vec2 {
    x.scale(snapping).round().inv_scale(snapping)
}

fn calculate_bounding_box(verts: &[BatchVertex]) -> Rect {
    let mut min = vec2::one(f32::INFINITY);
    let mut max = vec2::one(f32::NEG_INFINITY);

    for v in verts {
        min = min.min(v.pos);
        max = max.max(v.pos);
    }

    return Rect { start: min, end: max };
}

fn aabb_check(a: Rect, b: Rect) -> bool {
    return
        a.start.0 < b.end.0 && b.start.0 < a.end.0 &&
        a.start.1 < b.end.1 && b.start.1 < a.end.1;
}
