use nogine2_core::{assert_expr, main_thread::test_main_thread, math::{mat3x3::mat3, rect::Rect, vector2::{uvec2, vec2}, vector3::vec3}};

use crate::{colors::{rgba::RGBA32, Color}, graphics::{batch::BatchPushCmd, pipeline::SceneData, texture::rendertex::RenderTexture, vertex::BatchVertex}};

use super::{batch::BatchData, blending::BlendingMode, pipeline::{DefaultPipeline, RenderPipeline, RenderStats}, texture::TextureHandle, CameraData, Graphics};

static DEFAULT_PIPELINE: DefaultPipeline = DefaultPipeline;

pub struct RenderScope {
    batch_data: BatchData,
    tex_ppu: f32,
    blending: BlendingMode,
    pivot: vec2,
    user_data: i32,

    render_started: bool,
    clear_col: RGBA32,
    pipeline: Option<PipelinePtr>,
}

impl RenderScope {
    pub const fn new() -> Self {
        Self {
            batch_data: BatchData::new(),
            tex_ppu: 1.0,
            blending: BlendingMode::AlphaMix,
            pivot: vec2::ZERO,
            user_data: 0,

            render_started: false,
            clear_col: RGBA32::BLACK,
            pipeline: None,
        }
    }

    /// Makes this scope the target for all render commands.
    pub fn run(&mut self, rt: &RenderTexture, setup: ScopeRenderSetup<'_>, f: impl FnOnce()) -> RenderStats {
        let pipeline = if let Some(pipeline) = setup.pipeline {
            unsafe { std::mem::transmute::<_, *const dyn RenderPipeline>(pipeline) } // Hack to stop misdiagnosis from rust (?)
        } else {
            &DEFAULT_PIPELINE as *const dyn RenderPipeline
        };

        self.begin_render(setup.camera, rt.dims(), setup.clear_col, pipeline);
        Graphics::swap_scope(self);
        f();
        Graphics::swap_scope(self);
        return self.end_render(rt);
    }

    pub(crate) fn draw_rect(&mut self, cmd: RectSubmitCmd) {
        test_main_thread();
        assert_expr!(self.render_started, "Render commands can only be called after Window::pre_tick!");

        let tf_mat = mat3::tf_matrix(cmd.pos, cmd.rot, cmd.extents.scale(vec2(1.0, -1.0)));

        let user_data = self.user_data;
        let verts = &[
            BatchVertex { pos: (&tf_mat * vec3::from_xy(vec2(0.0, 0.0) - self.pivot, 1.0)).xy(), tint: cmd.tint[0], uv: cmd.uv_rect.lu(), tex_id: 0, user_data },
            BatchVertex { pos: (&tf_mat * vec3::from_xy(vec2(0.0, 1.0) - self.pivot, 1.0)).xy(), tint: cmd.tint[1], uv: cmd.uv_rect.ld(), tex_id: 0, user_data },
            BatchVertex { pos: (&tf_mat * vec3::from_xy(vec2(1.0, 1.0) - self.pivot, 1.0)).xy(), tint: cmd.tint[2], uv: cmd.uv_rect.rd(), tex_id: 0, user_data },
            BatchVertex { pos: (&tf_mat * vec3::from_xy(vec2(1.0, 0.0) - self.pivot, 1.0)).xy(), tint: cmd.tint[3], uv: cmd.uv_rect.ru(), tex_id: 0, user_data },
        ];
        let indices = &[0, 1, 2, 2, 3, 0];
   
        let blending = self.blending;
        self.batch_data.push(BatchPushCmd::Triangles { verts, indices, texture: cmd.texture, blending });
    }

    pub(crate) fn draw_points(&mut self, cmd: PointsSubmitCmd<'_>) {
        test_main_thread();
        assert_expr!(self.render_started, "Render commands can only be called after Window::pre_tick!");

        let verts = cmd.points.iter().map(|(pos, col)|
            BatchVertex { pos: *pos, tint: *col, uv: vec2::ZERO, tex_id: 0, user_data: self.user_data }
        ).collect::<Vec<_>>();

        let blending = self.blending;
        self.batch_data.push(BatchPushCmd::Points { verts: &verts, blending });
    }

    pub(crate) fn draw_lines(&mut self, cmd: LineSubmitCmd) {
        test_main_thread();
        assert_expr!(self.render_started, "Render commands can only be called after Window::pre_tick!");

        let user_data = self.user_data;
        let verts = [
            BatchVertex { pos: cmd.verts[0], tint: cmd.cols[0], uv: vec2::ZERO, tex_id: 0, user_data },
            BatchVertex { pos: cmd.verts[1], tint: cmd.cols[1], uv: vec2::ZERO, tex_id: 0, user_data },
        ];

        let blending = self.blending;
        self.batch_data.push(BatchPushCmd::Lines { verts, blending });
    }

    /// Returns the current camera data.
    pub fn camera(&self) -> CameraData {
        return self.batch_data.camera();
    }

    /// Returns the pixels per unit for textures.
    pub fn pixels_per_unit(&self) -> f32 {
        return self.tex_ppu;
    }

    /// Sets the pixels per unit for textures. Will panic if `ppu <= 0.0`.
    pub fn set_pixels_per_unit(&mut self, ppu: f32) {
        assert_expr!(ppu > 0.0, "Pixels per unit for textures must be greater than 0!");
        self.tex_ppu = ppu;
    }

    /// Returns the user data.
    pub fn user_data(&self) -> i32 {
        return self.user_data;
    }

    /// Sets user data.
    pub fn set_user_data(&mut self, user_data: i32) {
        self.user_data = user_data;
    }

    /// Returns the current pivot.
    pub fn pivot(&self) -> vec2 {
        return self.pivot;
    }

    /// Sets the current pivot.
    pub fn set_pivot(&mut self, pivot: vec2) {
        self.pivot = pivot;
    }

    /// Returns the active blending mode.
    pub fn blending_mode(&self) -> BlendingMode {
        return self.blending;
    }

    /// Sets the active blending mode.
    pub fn set_blending_mode(&mut self, blending: BlendingMode) {
        self.blending = blending;
    }


    pub(crate) fn begin_render(&mut self, camera: CameraData, target_res: uvec2, clear_col: RGBA32, pipeline: *const dyn RenderPipeline) {
        self.batch_data.setup_frame(camera, target_res);
        self.render_started = true;
        self.pipeline = Some(PipelinePtr(pipeline));
        self.clear_col = clear_col;
    }

    pub(crate) fn end_render(&mut self, rt: &RenderTexture) -> RenderStats { 
        assert_expr!(self.render_started, "Window::post_tick must be called after Window::pre_tick!");
        self.render_started = false;

        let mut stats = RenderStats::new();
        let render_pipeline = unsafe { self.pipeline.as_ref().unwrap().0.as_ref().unwrap() };
        render_pipeline.render(rt, SceneData::new(&self.batch_data), self.clear_col, &mut stats);
        return stats;
    }
}


/// Holds all the required information to render with a scope.
pub struct ScopeRenderSetup<'a> {
    pub camera: CameraData,
    pub clear_col: RGBA32,
    pub pipeline: Option<&'a dyn RenderPipeline>,
}


pub(crate) struct RectSubmitCmd {
    pub pos: vec2,
    pub rot: f32,
    pub extents: vec2,
    pub tint: [RGBA32; 4],
    pub texture: TextureHandle,
    pub uv_rect: Rect,
}

pub(crate) struct PointsSubmitCmd<'a> {
    pub points: &'a [(vec2, RGBA32)],
}

pub(crate) struct LineSubmitCmd {
    pub verts: [vec2; 2],
    pub cols: [RGBA32; 2],
}


struct PipelinePtr(*const dyn RenderPipeline);
unsafe impl Sync for PipelinePtr {}
unsafe impl Send for PipelinePtr {}
