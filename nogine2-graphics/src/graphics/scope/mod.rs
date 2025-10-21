use std::sync::Arc;

use bitflags::bitflags;
use nogine2_core::{assert_expr, main_thread::test_main_thread, math::{lerp::Lerp, mat3x3::mat3, rect::Rect, vector2::{ivec2, uvec2, vec2}, vector3::vec3}};

use crate::{colors::{rgba::RGBA32, Color}, graphics::{batch::BatchPushCmd, pipeline::SceneData, text::{align::{HorTextAlign, VerTextAlign}, font::Font}, texture::rendertex::RenderTexture, vertex::BatchVertex}, TIME_TS};

use super::{batch::BatchData, blending::BlendingMode, defaults::DefaultMaterials, material::Material, pipeline::{DefaultPipeline, RenderPipeline, RenderStats}, text::{engine::{helpers::GraphicMetrics, TextEngine}, font::TextStyle, rich::{CharQuad, CharVert, RichTextContext}, TextCfg}, texture::{sprite::Sprite, TextureHandle}, CameraData, Graphics };

static DEFAULT_PIPELINE: DefaultPipeline = DefaultPipeline;

pub mod ui;

macro_rules! assert_pre_tick {
    ($obj:expr) => {
        assert_expr!(
            $obj.render_started,
            "Render commands can only be called after Window::pre_tick!"
        )
    };
}

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

    font_size: f32,
    hor_alignment: HorTextAlign,
    ver_alignment: VerTextAlign,
    word_wrap: bool,
    rich_text: bool,
    font_col: RGBA32,
    
    /// Indices of rtc in the current text being processed
    rich_text_commands: Vec<usize>,
    charquad_out: Vec<CharQuad>,
    charquad_in: Vec<CharQuad>,
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

            font_size: 12.0,
            hor_alignment: HorTextAlign::Left,
            ver_alignment: VerTextAlign::Top,
            word_wrap: false,
            rich_text: false,            
            font_col: RGBA32::WHITE,

            rich_text_commands: Vec::new(),
            charquad_out: Vec::new(),
            charquad_in: Vec::new(),
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
        assert_pre_tick!(self);
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
            BatchVertex { // Left Down
                pos: (&tf_mat * vec3::from_xy(vec2(0.0, 0.0) - self.pivot, 1.0)).xy(),
                tint: cmd.tint[0],
                uv: uvs[0],
                uv1: vec2(0.0, 1.0),
                tex_id: 0,
                user_data
            },
            BatchVertex { // Left Up
                pos: (&tf_mat * vec3::from_xy(vec2(0.0, 1.0) - self.pivot, 1.0)).xy(),
                tint: cmd.tint[1],
                uv: uvs[1],
                uv1: vec2(0.0, 0.0),
                tex_id: 0,
                user_data
            },
            BatchVertex { // Right Up
                pos: (&tf_mat * vec3::from_xy(vec2(1.0, 1.0) - self.pivot, 1.0)).xy(),
                tint: cmd.tint[2],
                uv: uvs[2],
                uv1: vec2(1.0, 0.0),
                tex_id: 0,
                user_data
            },
            BatchVertex { // Right Down
                pos: (&tf_mat * vec3::from_xy(vec2(1.0, 0.0) - self.pivot, 1.0)).xy(),
                tint: cmd.tint[3],
                uv: uvs[3],
                uv1: vec2(1.0, 1.0),
                tex_id: 0,
                user_data
            },
        ];
        let indices = &[0, 1, 2, 2, 3, 0];
   
        let blending = self.blending;
        let material = self.material();
        let culling_enabled = self.cfg_flags.contains(RenderScopeCfgFlags::CULLING);
        self.batch_data.push(BatchPushCmd::Triangles { verts, indices, texture: cmd.texture, blending, material }, culling_enabled);
    }

    pub(crate) fn draw_points(&mut self, cmd: PointsSubmitCmd<'_>) {
        test_main_thread();
        assert_pre_tick!(self);

        let y_scaling = if self.cfg_flags.contains(RenderScopeCfgFlags::POSITIVE_Y_IS_DOWN) { -1.0 } else { 1.0 };
        let verts = cmd.points.iter().map(|(pos, col)|
            BatchVertex {
                pos: (*pos).scale(vec2(1.0, y_scaling)),
                tint: *col,
                user_data: self.user_data,
                ..Default::default()
            }
        ).collect::<Vec<_>>();

        let blending = self.blending;
        let material = self.material();
        let culling_enabled = self.cfg_flags.contains(RenderScopeCfgFlags::CULLING);
        self.batch_data.push(BatchPushCmd::Points { verts: &verts, blending, material }, culling_enabled);
    }

    pub(crate) fn draw_line(&mut self, cmd: LineSubmitCmd) {
        test_main_thread();
        assert_pre_tick!(self);

        let y_scaling = if self.cfg_flags.contains(RenderScopeCfgFlags::POSITIVE_Y_IS_DOWN) { -1.0 } else { 1.0 };
        let user_data = self.user_data;
        let verts = [
            BatchVertex {
                pos: cmd.verts[0].scale(vec2(1.0, y_scaling)),
                tint: cmd.cols[0],
                user_data,
                ..Default::default()
            },
            BatchVertex {
                pos: cmd.verts[1].scale(vec2(1.0, y_scaling)),
                tint: cmd.cols[1],
                user_data,
                ..Default::default()
            },
        ];

        let blending = self.blending;
        let material = self.material();
        let culling_enabled = self.cfg_flags.contains(RenderScopeCfgFlags::CULLING);
        self.batch_data.push(BatchPushCmd::Lines { verts, blending, material }, culling_enabled);
    }

    pub(crate) fn draw_9_patch(&mut self, cmd: NinePatchSubmitCmd) {
        fn bilinear(rect: Rect, mut uv: vec2, inverted_y: bool) -> vec2 {
            if !inverted_y {
                uv.1 = 1.0 - uv.1;
            }
            
            let up = rect.lu().lerp(rect.ru(), uv.0);
            let down = rect.ld().lerp(rect.rd(), uv.0);
            return down.lerp(up, uv.1);
        }
        
        test_main_thread();
        assert_pre_tick!(self);

        let corner_size = vec2::from(cmd.sprite.dims()) / self.tex_ppu * cmd.corner_scaling / 3.0;
        let min_extents = corner_size * 2.0;
        let extents = cmd.extents.max(min_extents);
        let rel_corner = corner_size.inv_scale(extents);
        
        let inverted_y = self.cfg_flags.contains(RenderScopeCfgFlags::POSITIVE_Y_IS_DOWN);

        let y_scaling = if inverted_y { -1.0 } else { 1.0 };
        let tf_mat = mat3::tf_matrix(
            cmd.pos.scale(vec2(1.0, y_scaling)),
            cmd.rot,
            extents.scale(vec2(1.0, -y_scaling))
        );

        let uv_rect = cmd.sprite.uv_rect();
        let user_data = self.user_data;
        let diag = [vec2::ZERO, rel_corner, vec2::ONE - rel_corner, vec2::ONE];
        let mut verts = [BatchVertex::default(); 16];
        for i in 0..16 {
            let mut uv = vec2::from(ivec2(i as i32 % 4, i as i32 / 4)) / 3.0;
            uv.1 = 1.0 - uv.1;

            let pos = diag[i % 4].xvec() + diag[i / 4].yvec();
            verts[i] = BatchVertex {
                pos: (&tf_mat * vec3::from_xy(pos - self.pivot, 1.0)).xy(),
                tint: cmd.tint,
                uv: bilinear(uv_rect, uv, inverted_y),
                uv1: uv,
                tex_id: 0,
                user_data
            };
        }

        // 12  13  14  15
        //  8   9  10  11
        //  4   5   6   7
        //  0   1   2   3
        let indices = &[
            0, 4, 5, 5, 1, 0,
            1, 5, 6, 6, 2, 1,
            2, 6, 7, 7, 3, 2,
            4, 8, 9, 9, 5, 4,
            5, 9, 10, 10, 6, 5,
            6, 10, 11, 11, 7, 6,
            8, 12, 13, 13, 9, 8,
            9, 13, 14, 14, 10, 9,
            10, 14, 15, 15, 11, 10,
        ];
   
        let blending = self.blending;
        let material = self.material();
        let culling_enabled = self.cfg_flags.contains(RenderScopeCfgFlags::CULLING);
        self.batch_data.push(BatchPushCmd::Triangles {
            verts: &verts,
            indices,
            texture: cmd.sprite.handle().clone(),
            blending,
            material
        }, culling_enabled);
    }

    pub(crate) fn draw_text(
        &mut self,
        origin: vec2,
        rot: f32,
        extents: vec2,
        text: &str,
        font: &dyn Font
    ) {
        return self.draw_text_stateless(
            TextCfg {
                origin,
                rot,
                scale: vec2::ONE,
                extents,
                font_size: self.font_size,
                font_col: self.font_col,
                font,
                hor_alignment: self.hor_alignment,
                ver_alignment: self.ver_alignment,
                word_wrap: self.word_wrap,
                rich_text: self.rich_text,
                progress: None,
            },
            text
        );
    }

    pub(crate) fn draw_text_stateless(&mut self, cfg: TextCfg, text: &str) {
        test_main_thread();
        assert_pre_tick!(self);

        let GraphicMetrics {
            font_size,
            line_height,
            char_separation,
            space_width
        } = GraphicMetrics::calculate(&cfg, self.tex_ppu);

        self.text_engine.load(text, &cfg, self.tex_ppu);

        let mut sanitized_text = String::new(); // MUST BE EMPTY
        let mut rt_stack = Vec::new(); // MUST BE EMPTY
        let mut rt_args_stack = String::new(); // MUST BE EMPTY
        self.text_engine.swap_sanitized_text(&mut sanitized_text); // MUST BE SWAPPED BACK
        self.text_engine.swap_rt_stack(&mut rt_stack); // MUST BE SWAPPED BACK
        self.text_engine.swap_rt_args_stack(&mut rt_args_stack); // MUST BE SWAPPED BACK

        // Index of rich text commands to check
        self.rich_text_commands.clear();
        let mut rt_index = 0usize;
        let mut char_index = 0usize;
        let mut pindex = 0usize;
        
        let (dy0, mut line_separation) = cfg.ver_alignment.dy0_and_spaces(
            cfg.extents.1,
            line_height,
            self.text_engine.get_line_count(),
        );

        line_separation = line_separation.max(0.0);

        let mut charquad_in = Vec::new();
        let mut charquad_out = Vec::new();

        std::mem::swap(&mut charquad_in, &mut self.charquad_in);
        std::mem::swap(&mut charquad_out, &mut self.charquad_out);

        self.text_engine.advance_y(dy0);
        'outer: for (i, line) in sanitized_text.lines().enumerate() {
            let (dx0, mut space_width) = cfg.hor_alignment.dx0_and_spaces(
                cfg.extents.0,
                space_width,
                &self.text_engine.get_line_data(i)
            );
            space_width = space_width.max(0.0);

            self.text_engine.advance_x(dx0);
            
            for c in line.chars() {
                if let Some(max) = cfg.progress {
                    if pindex >= max {
                        break 'outer;
                    }
                }
                
                // Rich Text Command thingy
                while let Some(x) = rt_stack.get(rt_index) {
                    if x.char_index <= char_index {
                        match x.index {
                            Some(cmd) => { self.rich_text_commands.push(cmd); },
                            None => { self.rich_text_commands.pop(); },
                        };
                        rt_index += 1;
                    } else {
                        break;
                    }
                }
                
                if c.is_whitespace() {
                    // No char_separation because it is already included in space_width
                    self.text_engine.advance_x(space_width);
                    char_index += c.len_utf8();
                    pindex += 1;
                    continue;
                }
            
                let style = self.rich_text_commands.iter()
                    .scan(TextStyle::Regular, |state, id| {
                        *state = cfg.font.get_rich_functions()[*id].new_style(*state);
                        return Some(*state);
                    })
                    .last()
                    .unwrap_or(TextStyle::Regular);

                if let Some((sprite, _)) = cfg.font.get_char(style, c) {
                    // <<<<<<<<<<<<<<<<<<<<<< R I G H T      H E R E >>>>>>>>>>>>>>>>>>>>>>>>>> //
                    // TODO: This two lines are probably very dumb
                    let scale = font_size / sprite.dims().1 as f32;
                    let height = sprite.dims().1 as f32 * scale;
                    let initial = initial_charquad(
                        vec2::ZERO,
                        &sprite,
                        scale,
                        cfg.font_col,
                    );

                    let (time, ts) = {
                        let time_ts = TIME_TS.lock().unwrap();
                        *time_ts
                    };

                    let ctx = RichTextContext {
                        time,
                        ts,
                        index: char_index,
                        char: c,
                    };

                    charquad_out.clear();
                    charquad_out.push(initial);

                    for cmd in &self.rich_text_commands {
                        charquad_in.clear();
                        charquad_in.extend_from_slice(&charquad_out);

                        cfg.font.get_rich_functions()[*cmd].draw(
                            rt_args_stack.lines().nth(*cmd).unwrap_or("").split(','),
                            &charquad_in,
                            &mut charquad_out,
                            &ctx,
                        );
                    }

                    self.text_engine.add_quads(&charquad_out, height);

                    let width = sprite.dims().0 as f32 / sprite.dims().1 as f32 * font_size;
                    self.text_engine.advance_x(width + char_separation);
                    // <<<<<<<<<<<<<<<<<<<<<<========================>>>>>>>>>>>>>>>>>>>>>>>>>> //
                }
                char_index += c.len_utf8();
                pindex += 1;
            }
            self.text_engine.advance_y(line_separation);
            char_index += 1;
            pindex += 1;
        }

        self.text_engine.swap_sanitized_text(&mut sanitized_text); // Return the real buffer
        self.text_engine.swap_rt_stack(&mut rt_stack); // Return the real buffer
        self.text_engine.swap_rt_args_stack(&mut rt_args_stack); // Return the real buffer

        std::mem::swap(&mut charquad_in, &mut self.charquad_in);
        std::mem::swap(&mut charquad_out, &mut self.charquad_out);

        let culling_enabled = self.cfg_flags.contains(RenderScopeCfgFlags::CULLING);
        let material = self.material();

        let inverted_y = self.cfg_flags.contains(RenderScopeCfgFlags::POSITIVE_Y_IS_DOWN);

        let y_scaling = if inverted_y { -1.0 } else { 1.0 };
        let tf_mat = mat3::tf_matrix(
            cfg.origin.scale(vec2(1.0, y_scaling))
            - if inverted_y { cfg.extents.yvec().scale(cfg.scale) } else { vec2::ZERO }
            - cfg.extents.scale(cfg.scale).scale(self.pivot).scale(vec2(1.0, y_scaling)),
            cfg.rot,
            cfg.scale.scale(vec2(1.0, -1.0))
        );

        self.text_engine.render(
            &mut self.batch_data,
            tf_mat,
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

    /// Returns the font size.
    pub fn font_size(&self) -> f32 {
        self.font_size
    }

    /// Sets the font size.
    pub fn set_font_size(&mut self, font_size: f32) {
        self.font_size = font_size;
    }

    /// Returns the font col.
    pub fn font_col(&self) -> RGBA32 {
        self.font_col
    }

    /// Sets the font col.
    pub fn set_font_col(&mut self, font_col: RGBA32) {
        self.font_col = font_col;
    }

    /// Returns the horizontal alignment for text.
    pub fn text_hor_alignment(&self) -> HorTextAlign {
        self.hor_alignment
    }

    /// Sets the horizontal alignment for text.
    pub fn set_text_hor_alignment(&mut self, text_hor_alignment: HorTextAlign) {
        self.hor_alignment = text_hor_alignment;
    }

    /// Returns the vertical alignment for text.
    pub fn text_ver_alignment(&self) -> VerTextAlign {
        self.ver_alignment
    }

    /// Sets the vertical alignment for text.
    pub fn set_text_ver_alignment(&mut self, text_ver_alignment: VerTextAlign) {
        self.ver_alignment = text_ver_alignment;
    }

    /// Returns the word wrap flag.
    pub fn word_wrap(&self) -> bool {
        self.word_wrap
    }

    /// Sets the word wrap flag.
    pub fn set_word_wrap(&mut self, word_wrap: bool) {
        self.word_wrap = word_wrap;
    }

    /// Returns the rich text flag.
    pub fn rich_text(&self) -> bool {
        self.rich_text
    }

    /// Sets the rich text flag.
    pub fn set_rich_text(&mut self, rich_text: bool) {
        self.rich_text = rich_text;
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

    /// Returns the target resolution.
    pub fn target_res(&self) -> uvec2 {
        self.batch_data.target_res()
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
        assert_pre_tick!(self);
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

pub(crate) struct NinePatchSubmitCmd {
    pub pos: vec2,
    pub rot: f32,
    pub extents: vec2,
    pub tint: RGBA32,
    pub sprite: Sprite,
    pub corner_scaling: f32,
}


struct PipelinePtr(*const dyn RenderPipeline);
unsafe impl Sync for PipelinePtr {}
unsafe impl Send for PipelinePtr {}

fn initial_charquad(offset: vec2, sprite: &Sprite, scale: f32, color: RGBA32) -> CharQuad {
    let rect = Rect::from_points(
        offset,
        offset + vec2::from(sprite.dims()) * scale,
    );

    return CharQuad {
        ld: CharVert { pos: rect.ld(), color, user_data: 0 },
        lu: CharVert { pos: rect.lu(), color, user_data: 0 },
        ru: CharVert { pos: rect.ru(), color, user_data: 0 },
        rd: CharVert { pos: rect.rd(), color, user_data: 0 },
        sprite: sprite.clone(),
    };
}
