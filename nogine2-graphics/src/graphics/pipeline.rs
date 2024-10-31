use nogine2_core::bytesize::ByteSize;

#[derive(Debug, Clone)]
pub struct RenderStats {
    /// Holds all the information related to the batch renderer.
    pub batch: BatchRenderStats,   
}

impl RenderStats {
    pub const fn new() -> Self {
        Self { batch: BatchRenderStats::new() }
    }

    pub fn total_draw_calls(&self) -> usize {
        self.batch.draw_calls
    }
}


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
