use nogine2_core::{assert_expr, main_thread::test_main_thread, math::{mat3x3::mat3, rect::Rect, vector2::{uvec2, vec2}, vector3::vec3}};

use crate::{colors::{rgba::RGBA32, Color}, graphics::{batch::BatchPushCmd, pipeline::SceneData, texture::rendertex::RenderTexture, vertex::BatchVertex}};

use super::{batch::BatchData, blending::BlendingMode, pipeline::{RenderPipeline, RenderStats}, texture::TextureHandle, CameraData};

pub struct RenderScope {
    batch_data: BatchData,
    tex_ppu: f32,
    blending: BlendingMode,

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

            render_started: false,
            clear_col: RGBA32::BLACK,
            pipeline: None,
        }
    }

    pub(crate) fn draw_rect(&mut self, cmd: RectSubmitCmd) {
        test_main_thread();
        assert_expr!(self.render_started, "Render commands can only be called after Window::pre_tick!");

        let tf_mat = mat3::tf_matrix(cmd.pos, cmd.rot, cmd.extents.scale(vec2(1.0, -1.0)));

        let verts = &[
            BatchVertex { pos: (&tf_mat * vec3(0.0, 0.0, 1.0)).xy(), tint: cmd.tint[0], uv: cmd.uv_rect.lu(), tex_id: 0, user_data: 0 },
            BatchVertex { pos: (&tf_mat * vec3(0.0, 1.0, 1.0)).xy(), tint: cmd.tint[1], uv: cmd.uv_rect.ld(), tex_id: 0, user_data: 0 },
            BatchVertex { pos: (&tf_mat * vec3(1.0, 1.0, 1.0)).xy(), tint: cmd.tint[2], uv: cmd.uv_rect.rd(), tex_id: 0, user_data: 0 },
            BatchVertex { pos: (&tf_mat * vec3(1.0, 0.0, 1.0)).xy(), tint: cmd.tint[3], uv: cmd.uv_rect.ru(), tex_id: 0, user_data: 0 },
        ];
        let indices = &[0, 1, 2, 2, 3, 0];
   
        let blending = self.blending;
        self.batch_data.push(BatchPushCmd { verts, indices, texture: cmd.texture, blending });
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

    pub(crate) fn end_render(&mut self, real_window_res: uvec2) -> RenderStats { 
        assert_expr!(self.render_started, "Window::post_tick must be called after Window::pre_tick!");
        self.render_started = false;

        let mut stats = RenderStats::new();
        let render_pipeline = unsafe { self.pipeline.as_ref().unwrap().0.as_ref().unwrap() };
        render_pipeline.render(&RenderTexture::to_screen(real_window_res), SceneData::new(&self.batch_data), self.clear_col, &mut stats);
        return stats;
    }
}

pub(crate) struct RectSubmitCmd {
    pub pos: vec2,
    pub rot: f32,
    pub extents: vec2,
    pub tint: [RGBA32; 4],
    pub texture: TextureHandle,
    pub uv_rect: Rect,
}


struct PipelinePtr(*const dyn RenderPipeline);
unsafe impl Sync for PipelinePtr {}
unsafe impl Send for PipelinePtr {}
