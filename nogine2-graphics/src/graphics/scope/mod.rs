use std::sync::Arc;

use bitflags::bitflags;
use nogine2_core::{assert_expr, main_thread::test_main_thread, math::{mat3x3::mat3, rect::Rect, vector2::{uvec2, vec2}, vector3::vec3}};

use crate::{colors::{rgba::RGBA32, Color}, graphics::{batch::BatchPushCmd, pipeline::SceneData, texture::rendertex::RenderTexture, vertex::BatchVertex}};

use super::{batch::BatchData, blending::BlendingMode, defaults::DefaultMaterials, material::Material, pipeline::{DefaultPipeline, RenderPipeline, RenderStats}, text::{engine::{helpers::GraphicMetrics, TextEngine}, font::{Measure, TextStyle}, TextCfg}, texture::TextureHandle, CameraData, Graphics, WHITE_TEX};

static DEFAULT_PIPELINE: DefaultPipeline = DefaultPipeline;

pub mod ui;

/// Basic render scope for scene rendering.
pub struct RenderScope {
    batch_data: BatchData,
    tex_ppu: f32,
    blending: BlendingMode,
    pivot: vec2,
    user_data: i32,
    material: Option<Arc<Material>>,

    text_engine: TextEngine,
    
    cfg_flags: RenderScopeCfgFlags,

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
            material: None,

            text_engine: TextEngine::new(),
            
            cfg_flags: RenderScopeCfgFlags::DEFAULT,

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
        return self.end_render(rt, false, None);
    }

    pub(crate) fn draw_rect(&mut self, cmd: RectSubmitCmd) {
        test_main_thread();
        assert_expr!(self.render_started, "Render commands can only be called after Window::pre_tick!");
        let inverted_y = self.cfg_flags.contains(RenderScopeCfgFlags::POSITIVE_Y_IS_DOWN);

        let y_scaling = if inverted_y { -1.0 } else { 1.0 };
        let tf_mat = mat3::tf_matrix(cmd.pos.scale(vec2(1.0, y_scaling)), cmd.rot, cmd.extents.scale(vec2(1.0, -y_scaling)));

        let uvs = if inverted_y {
            [cmd.uv_rect.ld(), cmd.uv_rect.lu(), cmd.uv_rect.ru(), cmd.uv_rect.rd()]
        } else {
            [cmd.uv_rect.lu(), cmd.uv_rect.ld(), cmd.uv_rect.rd(), cmd.uv_rect.ru()]
        };
    
        let user_data = self.user_data;
        let verts = &[
            BatchVertex { pos: (&tf_mat * vec3::from_xy(vec2(0.0, 0.0) - self.pivot, 1.0)).xy(), tint: cmd.tint[0], uv: uvs[0], tex_id: 0, user_data },
            BatchVertex { pos: (&tf_mat * vec3::from_xy(vec2(0.0, 1.0) - self.pivot, 1.0)).xy(), tint: cmd.tint[1], uv: uvs[1], tex_id: 0, user_data },
            BatchVertex { pos: (&tf_mat * vec3::from_xy(vec2(1.0, 1.0) - self.pivot, 1.0)).xy(), tint: cmd.tint[2], uv: uvs[2], tex_id: 0, user_data },
            BatchVertex { pos: (&tf_mat * vec3::from_xy(vec2(1.0, 0.0) - self.pivot, 1.0)).xy(), tint: cmd.tint[3], uv: uvs[3], tex_id: 0, user_data },
        ];
        let indices = &[0, 1, 2, 2, 3, 0];
   
        let blending = self.blending;
        let material = self.material();
        let culling_enabled = self.cfg_flags.contains(RenderScopeCfgFlags::CULLING);
        self.batch_data.push(BatchPushCmd::Triangles { verts, indices, texture: cmd.texture, blending, material }, culling_enabled);
    }

    pub(crate) fn draw_points(&mut self, cmd: PointsSubmitCmd<'_>) {
        test_main_thread();
        assert_expr!(self.render_started, "Render commands can only be called after Window::pre_tick!");

        let y_scaling = if self.cfg_flags.contains(RenderScopeCfgFlags::POSITIVE_Y_IS_DOWN) { -1.0 } else { 1.0 };
        let verts = cmd.points.iter().map(|(pos, col)|
            BatchVertex { pos: (*pos).scale(vec2(1.0, y_scaling)), tint: *col, uv: vec2::ZERO, tex_id: 0, user_data: self.user_data }
        ).collect::<Vec<_>>();

        let blending = self.blending;
        let material = self.material();
        let culling_enabled = self.cfg_flags.contains(RenderScopeCfgFlags::CULLING);
        self.batch_data.push(BatchPushCmd::Points { verts: &verts, blending, material }, culling_enabled);
    }

    pub(crate) fn draw_line(&mut self, cmd: LineSubmitCmd) {
        test_main_thread();
        assert_expr!(self.render_started, "Render commands can only be called after Window::pre_tick!");

        let y_scaling = if self.cfg_flags.contains(RenderScopeCfgFlags::POSITIVE_Y_IS_DOWN) { -1.0 } else { 1.0 };
        let user_data = self.user_data;
        let verts = [
            BatchVertex { pos: cmd.verts[0].scale(vec2(1.0, y_scaling)), tint: cmd.cols[0], uv: vec2::ZERO, tex_id: 0, user_data },
            BatchVertex { pos: cmd.verts[1].scale(vec2(1.0, y_scaling)), tint: cmd.cols[1], uv: vec2::ZERO, tex_id: 0, user_data },
        ];

        let blending = self.blending;
        let material = self.material();
        let culling_enabled = self.cfg_flags.contains(RenderScopeCfgFlags::CULLING);
        self.batch_data.push(BatchPushCmd::Lines { verts, blending, material }, culling_enabled);
    }

    pub(crate) fn draw_text(&mut self, cfg: TextCfg, text: &str) {
        test_main_thread();

        let GraphicMetrics {
            line_height,
            char_separation,
            space_width
        } = GraphicMetrics::calculate(&cfg, self.tex_ppu);

        self.text_engine.load(text, &cfg, self.tex_ppu);
        let mut sanitized_text = String::new(); // MUST BE EMPTY
        self.text_engine.swap_sanitized_text(&mut sanitized_text); // MUST BE SWAPPED BACK
        
        for (i, line) in sanitized_text.lines().enumerate() {
            let (dx0, space_width) = cfg.hor_alignment.dx0_and_spaces(
                cfg.extents.0,
                space_width,
                char_separation,
                &self.text_engine.get_line_data(i)
            );
            
            self.text_engine.advance_x(dx0);
            
            for c in line.chars() {
                if c.is_whitespace() {
                    self.text_engine.advance_x(2.0 * char_separation + space_width);
                    continue;
                }
            
                if let Some((sprite, _)) = cfg.font.get_char(TextStyle::Regular, c) {
                    self.text_engine.add_sprite(
                        vec2::ZERO,
                        &sprite,
                        line_height / sprite.dims().1 as f32
                    );

                    let width = sprite.dims().0 as f32 / sprite.dims().1 as f32 * line_height;
                    self.text_engine.advance_x(width + char_separation);
                }
            }
            self.text_engine.advance_y(line_height);
        }

        self.text_engine.swap_sanitized_text(&mut sanitized_text); // Return the real buffer

        let culling_enabled = self.cfg_flags.contains(RenderScopeCfgFlags::CULLING);
        let material = self.material();

        self.draw_rect(RectSubmitCmd {
            pos: cfg.origin,
            rot: cfg.rot,
            extents: cfg.extents,
            tint: [RGBA32::GRAY; 4],
            texture: WHITE_TEX.get(),
            uv_rect: Rect::IDENT
        });

        self.text_engine.render(
            &mut self.batch_data,
            mat3::tf_matrix(cfg.origin, cfg.rot, cfg.scale.scale(vec2(1.0, -1.0))),
            culling_enabled,
            self.blending,
            material,
        );
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

    /// Sets the active material.
    pub fn set_material(&mut self, material: Arc<Material>) {
        self.material = Some(material);
    }

    /// Resets the active material.
    pub fn reset_material(&mut self) {
        self.material = Some(DefaultMaterials::batch()); // This instead of None because it would be dumb not to!!
    }

    /// Returns the active material.
    pub fn material(&self) -> Arc<Material> {
        self.material.clone().unwrap_or(DefaultMaterials::batch())
    }

    /// Returns the current configuration.
    pub const fn cfg(&self) -> RenderScopeCfgFlags {
        self.cfg_flags
    }

    /// Enables the configurations in `flags`.
    pub fn enable_cfg(&mut self, flags: RenderScopeCfgFlags) {
        self.cfg_flags |= flags;
    }

    /// Disables the configurations in `flags`.
    pub fn disable_cfg(&mut self, flags: RenderScopeCfgFlags) {
        self.cfg_flags &= !flags;
    }

    /// Sets the configuration.
    pub const fn set_cfg(&mut self, flags: RenderScopeCfgFlags) {
        self.cfg_flags = flags;
    }
    

    pub(crate) fn begin_render(&mut self, mut camera: CameraData, target_res: uvec2, clear_col: RGBA32, pipeline: *const dyn RenderPipeline) {
        if self.cfg_flags.contains(RenderScopeCfgFlags::POSITIVE_Y_IS_DOWN) {
            camera.center.1 = -camera.center.1;
        }
        
        self.batch_data.setup_frame(camera, target_res);
        self.render_started = true;
        self.pipeline = Some(PipelinePtr(pipeline));
        self.clear_col = clear_col;
    }

    pub(crate) fn end_render(&mut self, rt: &RenderTexture, is_ui: bool, complement_data: Option<SceneData<'_>>) -> RenderStats { 
        assert_expr!(self.render_started, "Window::post_tick must be called after Window::pre_tick!");
        self.render_started = false;

        let mut stats = RenderStats::new();
        let render_pipeline = unsafe { self.pipeline.as_ref().unwrap().0.as_ref().unwrap() };

        if is_ui {
            render_pipeline.render(rt, complement_data, Some(self.get_scene_data()), self.clear_col, &mut stats);
        } else {
            render_pipeline.render(rt, Some(self.get_scene_data()), complement_data, self.clear_col, &mut stats);
        }
        return stats;
    }

    fn get_scene_data(&self) -> SceneData<'_> {
        SceneData::new(&self.batch_data)
    }

    fn target_res(&self) -> uvec2 {
        self.batch_data.target_res()
    }
}


bitflags! {
    /// Bitflags for render scope configuration.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct RenderScopeCfgFlags : u8 {
        /// Culls render submits outside of the camera's bounds. Enabled by default.
        const CULLING = 1 << 0;

        /// Defines positive Y as down and negative Y as up. Enabled by default on UI scopes.
        const POSITIVE_Y_IS_DOWN = 1 << 1;

        /// Default configuration.
        const DEFAULT = Self::CULLING.bits();
        const DEFAULT_UI = Self::DEFAULT.bits() | Self::POSITIVE_Y_IS_DOWN.bits();
    }
}

impl Default for RenderScopeCfgFlags {
    fn default() -> Self {
        Self::DEFAULT
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
