use std::{mem::size_of, sync::Arc};

use lines::{LnsBatchBuffers, LnsBatchRenderCall};
use nogine2_core::{bytesize::ByteSize, math::{mat3x3::mat3, rect::Rect, vector2::{uvec2, vec2}}};
use points::{PtsBatchBuffers, PtsBatchRenderCall};
use triangles::{TriBatchBuffers, TriBatchRenderCall};


use super::{blending::BlendingMode, material::Material, pipeline::BatchRenderStats, texture::TextureHandle, vertex::BatchVertex, CameraData};

mod triangles;
mod points;
mod lines;

pub struct BatchData {
    render_calls: Vec<BatchRenderCall>,
    pooled_buffers: BuffersPool,

    view_mat: mat3,
    cam_rect: Rect,
    snapping: vec2,
    target_res: uvec2,
    camera: CameraData,

    stats: BatchRenderStats,
}

impl BatchData {
    pub const fn new() -> Self {
        Self {
            render_calls: Vec::new(), pooled_buffers: BuffersPool::new(),
            view_mat: mat3::IDENTITY, cam_rect: Rect::IDENT, snapping: vec2::ONE, camera: CameraData { center: vec2::ZERO, extents: vec2::ZERO }, target_res: uvec2::ZERO,
            stats: BatchRenderStats::new(),
        }
    }

    pub fn push(&mut self, cmd: BatchPushCmd<'_>, culling_enabled: bool) {
        match cmd {
            BatchPushCmd::Triangles { verts, indices, texture, blending, material } => {
                if culling_enabled {
                    let bb = calculate_bounding_box(verts);
                    if !aabb_check(self.cam_rect, bb) {
                        self.stats.skipped_submissions += 1;
                        return;
                    }
                }
                self.stats.rendered_submissions += 1;

                let mut verts = verts.iter().copied().map(|mut x| {
                    x.pos = snap(x.pos, self.snapping);
                    return x;
                }).collect::<Vec<_>>();
                let mut indices = indices.to_vec();

                self.stats.verts += verts.len();
                self.stats.triangles += indices.len() / 3;

                let cursor = self.tri_render_call_cursor(verts.len(), indices.len(), &texture, blending, material);
                if let BatchRenderCall::Triangles(call) = &mut self.render_calls[cursor] {
                    call.push(&mut verts, &mut indices, texture);
                }
            },
            BatchPushCmd::Points { verts, blending, material } => {
                if culling_enabled {
                    let bb = calculate_bounding_box(verts);
                    if !aabb_check(self.cam_rect, bb) {
                        self.stats.skipped_submissions += 1;
                        return;
                    }
                }
                self.stats.rendered_submissions += 1;
 
                let mut verts = verts.iter().copied().map(|mut x| {
                    x.pos = snap(x.pos, self.snapping);
                    return x;
                }).collect::<Vec<_>>();

                self.stats.verts += verts.len();

                let cursor = self.pts_render_call_cursor(verts.len(), blending, material);
                if let BatchRenderCall::Points(call) = &mut self.render_calls[cursor] {
                    call.push(&mut verts);
                }
            },
            BatchPushCmd::Lines { mut verts, blending, material } => {
                if culling_enabled {
                    let bb = calculate_bounding_box(&verts);
                    if !aabb_check(self.cam_rect, bb) {
                        self.stats.skipped_submissions += 1;
                        return;
                    }
                }
                self.stats.rendered_submissions += 1;

                verts[0].pos = snap(verts[0].pos, self.snapping);
                verts[1].pos = snap(verts[1].pos, self.snapping);

                self.stats.verts += verts.len();
                self.stats.triangles += 2;

                let cursor = self.lns_render_call_cursor(verts.len(), 2, blending, material);
                if let BatchRenderCall::Lines(call) = &mut self.render_calls[cursor] {
                    call.push(verts);
                }
            },
        }
    }

    pub fn setup_frame(&mut self, mut camera: CameraData, target_res: uvec2) {
        let snapping = vec2::from(target_res).inv_scale(camera.extents);
        camera.center = snap(camera.center, snapping);

        self.snapping = snapping;
        self.view_mat = mat3::tf_matrix(camera.center, 0.0, camera.extents.scale(vec2(1.0, -1.0) * 0.5)).inverse().unwrap_or(mat3::IDENTITY);
        self.cam_rect = Rect { start: camera.center - camera.extents * 0.5, end: camera.center + camera.extents * 0.5 };
        self.camera = camera;
        self.stats = BatchRenderStats::new();
        self.target_res = target_res;

        self.clear();
    }

    fn clear(&mut self) {
        self.pooled_buffers.clear();
        while let Some(call) = self.render_calls.pop() {
            match call {
                BatchRenderCall::Triangles(call) => self.pooled_buffers.push_tri_buffer(call.recycle()),
                BatchRenderCall::Points(call) => self.pooled_buffers.push_pts_buffer(call.recycle()),
                BatchRenderCall::Lines(call) => self.pooled_buffers.push_lns_buffer(call.recycle()),
            }
        }
    }

    pub fn render(&self, stats: &mut BatchRenderStats) {
        let mut on_use_size = 0;
        for call in &self.render_calls {
            call.render(&self.view_mat);
            stats.draw_calls += 1;
            on_use_size += call.on_use_size();
        }

        stats.allocated_memory = ByteSize::new((self.render_calls.iter().map(|x| x.alloc_size()).sum::<usize>() + self.pooled_buffers.buffer_sizes()) as u64);
        stats.on_use_memory = ByteSize::new(on_use_size as u64);

        *stats = stats.clone() + self.stats.clone();
    } 

    pub fn camera(&self) -> CameraData {
        self.camera.clone()
    }

    pub fn target_res(&self) -> uvec2 {
        self.target_res
    }

    fn tri_render_call_cursor(&mut self, verts_len: usize, indices_len: usize, texture: &TextureHandle, blending: BlendingMode, material: Arc<Material>) -> usize {
        if let Some(BatchRenderCall::Triangles(last)) = self.render_calls.last() {
            if last.allows(verts_len, indices_len, texture, blending, &material) {
                return self.render_calls.len() - 1;
            }
        }
        let buffers = self.pooled_buffers.get_tri_buffer();
        self.render_calls.push(BatchRenderCall::Triangles(TriBatchRenderCall::new(buffers, blending, material)));
        return self.render_calls.len() - 1;
    }

    fn pts_render_call_cursor(&mut self, verts_len: usize, blending: BlendingMode, material: Arc<Material>) -> usize {
        if let Some(BatchRenderCall::Points(last)) = self.render_calls.last() {
            if last.allows(verts_len, blending, &material) {
                return self.render_calls.len() - 1;
            }
        }
        let buffers = self.pooled_buffers.get_pts_buffer();
        self.render_calls.push(BatchRenderCall::Points(PtsBatchRenderCall::new(buffers, blending, material)));
        return self.render_calls.len() - 1;
    }

    fn lns_render_call_cursor(&mut self, verts_len: usize, indices_len: usize, blending: BlendingMode, material: Arc<Material>) -> usize {
        if let Some(BatchRenderCall::Lines(last)) = self.render_calls.last() {
            if last.allows(verts_len, indices_len, blending, &material) {
                return self.render_calls.len() - 1;
            }
        }
        let buffers = self.pooled_buffers.get_lns_buffer();
        self.render_calls.push(BatchRenderCall::Lines(LnsBatchRenderCall::new(buffers, blending, material)));
        return self.render_calls.len() - 1;
    }
}


pub enum BatchPushCmd<'a> {
    Triangles {
        verts: &'a [BatchVertex],
        indices: &'a [u16],
        texture: TextureHandle,
        blending: BlendingMode,
        material: Arc<Material>,
    },
    Points {
        verts: &'a [BatchVertex],
        blending: BlendingMode,
        material: Arc<Material>,
    },
    Lines {
        verts: [BatchVertex; 2],
        blending: BlendingMode,
        material: Arc<Material>,
    }
}


enum BatchRenderCall {
    Triangles(TriBatchRenderCall),
    Points(PtsBatchRenderCall),
    Lines(LnsBatchRenderCall),
}

impl BatchRenderCall {
    fn render(&self, view_mat: &mat3) {
        match self {
            BatchRenderCall::Triangles(call) => call.render(view_mat),
            BatchRenderCall::Points(call) => call.render(view_mat),
            BatchRenderCall::Lines(call) => call.render(view_mat),
        }
    }

    fn on_use_size(&self) -> usize {
        match self {
            BatchRenderCall::Triangles(call) => call.on_use_size(),
            BatchRenderCall::Points(call) => call.on_use_size(),
            BatchRenderCall::Lines(call) => call.on_use_size(),
        }
    }

    fn alloc_size(&self) -> usize {
        match self {
            BatchRenderCall::Triangles(_) => TriBatchBuffers::BYTE_SIZE,
            BatchRenderCall::Points(_) => PtsBatchBuffers::BYTE_SIZE,
            BatchRenderCall::Lines(_) => LnsBatchBuffers::BYTE_SIZE,
        }
    }
}


struct BuffersPool {
    tri_buffers: Vec<TriBatchBuffers>,
    pts_buffers: Vec<PtsBatchBuffers>,
    lns_buffers: Vec<LnsBatchBuffers>,
}

impl BuffersPool {
    const fn new() -> Self {
        Self { tri_buffers: Vec::new(), pts_buffers: Vec::new(), lns_buffers: Vec::new() }
    }

    fn clear(&mut self) {
        self.tri_buffers.clear();
        self.pts_buffers.clear();
        self.lns_buffers.clear();
    }

    fn buffer_sizes(&self) -> usize {
        const TRI_BATCH_BUFFER_SIZE: usize = size_of::<BatchVertex>() * TriBatchBuffers::MAX_VERTS + size_of::<u16>() * TriBatchBuffers::MAX_INDICES;
        const PTS_BATCH_BUFFER_SIZE: usize = size_of::<BatchVertex>() * PtsBatchBuffers::MAX_PTS;
        const LNS_BATCH_BUFFER_SIZE: usize = size_of::<BatchVertex>() * LnsBatchBuffers::MAX_VERTS + size_of::<u16>() * LnsBatchBuffers::MAX_INDICES;
        return self.tri_buffers.len() * TRI_BATCH_BUFFER_SIZE +
            self.pts_buffers.len() * PTS_BATCH_BUFFER_SIZE +
            self.lns_buffers.len() * LNS_BATCH_BUFFER_SIZE;
    }

    fn get_tri_buffer(&mut self) -> TriBatchBuffers {
        match self.tri_buffers.pop() {
            Some(x) => x,
            None => TriBatchBuffers::new(),
        }
    }

    fn push_tri_buffer(&mut self, buf: TriBatchBuffers) {
        self.tri_buffers.push(buf);
    }

    fn get_pts_buffer(&mut self) -> PtsBatchBuffers {
        match self.pts_buffers.pop() {
            Some(x) => x,
            None => PtsBatchBuffers::new(),
        }
    }

    fn push_pts_buffer(&mut self, buf: PtsBatchBuffers) {
        self.pts_buffers.push(buf);
    }

    fn get_lns_buffer(&mut self) -> LnsBatchBuffers {
        match self.lns_buffers.pop() {
            Some(x) => x,
            None => LnsBatchBuffers::new(),
        }
    }

    fn push_lns_buffer(&mut self, buf: LnsBatchBuffers) {
        self.lns_buffers.push(buf);
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
