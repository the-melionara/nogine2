use std::ops::Add;

use nogine2_core::{bytesize::ByteSize, log_error, main_thread::test_main_thread, math::{rect::IRect, vector2::ivec2}};

use crate::{colors::rgba::RGBA32, gl_wrapper::{framebuffer::GlFramebuffer, gl_viewport}};

use super::{batch::BatchData, texture::rendertex::RenderTexture};

/// Trait for customlizable render pipelines.
pub trait RenderPipeline {
    fn render(&self, target_rt: &RenderTexture, scene_data: SceneData<'_>, clear_col: RGBA32, stats: &mut RenderStats);
}


/// Holds all the required data for rendering a scene.
pub struct SceneData<'a> {
    batch_data: &'a BatchData,
}

impl<'a> SceneData<'a> {
    pub(crate) fn new(batch_data: &'a BatchData) -> Self {
        Self { batch_data }
    }

    /// Renders the scene data to a selected render texture.
    pub fn render_to(&self, rt: &RenderTexture, stats: &mut RenderStats) {
        test_main_thread();
    
        if rt.dims() != self.batch_data.target_res() {
            log_error!("Couldn't render! RenderTexture must have the specified target resolution to be able to render a scene!");
            return;
        }

        rt.bind();
        gl_viewport(IRect { start: ivec2::ZERO, end: rt.dims().into() });
        self.batch_data.render(&mut stats.batch);
        GlFramebuffer::to_screen().bind();
    }
}


/// Default pipeline for rendering.
#[derive(Debug, Clone, Copy)]
pub struct DefaultPipeline;

impl RenderPipeline for DefaultPipeline {
    fn render(&self, target_rt: &RenderTexture, scene_data: SceneData<'_>, clear_col: RGBA32, stats: &mut RenderStats) {
        target_rt.clear(clear_col);
        scene_data.render_to(target_rt, stats);
    }
}


/// Holds statistics about a rendered frame.
#[derive(Debug, Clone)]
pub struct RenderStats {
    /// Holds all the information related to the batch renderer.
    pub batch: BatchRenderStats,   

    /// Holds all the information related to blits.
    pub blit: BlitRenderStats,
}

impl RenderStats {
    pub const fn new() -> Self {
        Self { batch: BatchRenderStats::new(), blit: BlitRenderStats::new() }
    }

    pub fn total_draw_calls(&self) -> usize {
        self.batch.draw_calls
    }
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct BatchRenderStats {
    /// Number of batch draw calls performed in the frame.
    pub draw_calls: usize,

    /// Batch submissions that were frustum culled.
    pub skipped_submissions: usize,

    /// Batch submissions that were rendered.
    pub rendered_submissions: usize,

    /// Number of vertices rendered.
    pub verts: usize,

    /// Number of triangles rendered.
    pub triangles: usize,

    /// Allocated memory size.
    pub allocated_memory: ByteSize,

    /// Memory being used.
    pub on_use_memory: ByteSize,
}

impl BatchRenderStats {
    pub const fn new() -> Self {
        Self {
            draw_calls: 0, skipped_submissions: 0, rendered_submissions: 0, verts: 0, triangles: 0,
            allocated_memory: ByteSize::new(0), on_use_memory: ByteSize::new(0),
        }
    }

    pub fn total_submissions(&self) -> usize {
        self.skipped_submissions + self.rendered_submissions
    }
}

impl Add for BatchRenderStats {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            draw_calls: self.draw_calls + rhs.draw_calls,
            skipped_submissions: self.skipped_submissions + rhs.skipped_submissions,
            rendered_submissions: self.rendered_submissions + rhs.rendered_submissions,
            verts: self.verts + rhs.verts,
            triangles: self.triangles + rhs.triangles,
            allocated_memory: self.allocated_memory + rhs.allocated_memory,
            on_use_memory: self.on_use_memory + rhs.on_use_memory,
        }
    }
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct BlitRenderStats {
    /// Number of blit draw calls
    pub draw_calls: usize,
}

impl BlitRenderStats {
    pub const fn new() -> Self {
        Self { draw_calls: 0 }
    }
}
