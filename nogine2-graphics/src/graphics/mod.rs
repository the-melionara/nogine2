use std::sync::{Arc, RwLock};

use blending::BlendingMode;
use material::Material;
use nogine2_core::{crash, lazy::LazyCloner, math::{rect::Rect, vector2::{uvec2, vec2}}};
use pipeline::{RenderPipeline, RenderStats};
use scope::{ui::UIScope, LineSubmitCmd, PointsSubmitCmd, RectSubmitCmd, RenderScope, RenderScopeCfgFlags};
use text::TextCfg;
use texture::{pixels::{PixelFormat, Pixels}, rendertex::RenderTexture, sprite::Sprite, Texture2D, TextureFiltering, TextureHandle, TextureSampling, TextureWrapping};
use ui::area::UIArea;

use crate::{colors::{rgba::RGBA32, Color}, graphics::text::{align::{HorTextAlign, VerTextAlign}, font::Font}};

pub mod vertex;
pub mod defaults;
pub mod shader;
pub mod pipeline;
pub mod texture;
pub mod blending;
pub mod scope;
pub mod gfx;
pub mod material;
pub mod ui;
pub mod text;

mod batch;

pub(crate) static WHITE_TEX: LazyCloner<TextureHandle> = LazyCloner::new(|| Texture2D::new(
    Pixels::new(vec![255, 255, 255, 255], uvec2(1, 1), PixelFormat::RGBA8),
    TextureSampling { filtering: TextureFiltering::Nearest, wrapping: TextureWrapping::Clamp },
).handle());

static GRAPHICS: RwLock<Graphics> = RwLock::new(Graphics::new());

pub struct Graphics {
    active_scope: RenderScope,
    ui_scope: UIScope,
    ui_enabled: bool,
}

impl Graphics {
    const fn new() -> Self {
        Self {
            active_scope: RenderScope::new(),
            ui_scope: UIScope::new(),
            ui_enabled: false,
        }
    }

    /// Runs UI commands. Will panic if UI is not enabled.
    pub fn ui<'a, R>(f: impl FnOnce(UIArea<'a>) -> R) -> R {
        match Self::try_ui(f) {
            Some(res) => res,
            None => crash!("UI is not enabled!"),
        }
    }

    /// Runs UI commands. Will return `None` if UI is not enabled.
    pub fn try_ui<'a, R>(f: impl FnOnce(UIArea<'a>) -> R) -> Option<R> {
        let Ok(mut graphics) = GRAPHICS.write() else { crash!("Couldn't access Graphics singleton!") };
        if !graphics.ui_enabled {
            return None;
        }
        
        let hacky_ref = unsafe { (&mut graphics.ui_scope as *mut UIScope).as_mut().unwrap_unchecked() }; // there will be trials for my crimes
        return Some(UIScope::run_internal(hacky_ref, f));
    }

    pub fn draw_rect(pos: vec2, rot: f32, extents: vec2, color: RGBA32) {
        let Ok(mut graphics) = GRAPHICS.write() else { crash!("Couldn't access Graphics singleton!") };

        graphics.active_scope.draw_rect(RectSubmitCmd { pos, rot, extents, tint: [color; 4], texture: WHITE_TEX.get(), uv_rect: Rect::IDENT });
    }

    pub fn draw_texture(pos: vec2, rot: f32, scale: vec2, tint: RGBA32, texture: &Texture2D) {
        return Self::draw_texture_adv(pos, rot, scale, [tint; 4], texture.handle(), Rect::IDENT);
    }

    pub fn draw_sprite(pos: vec2, rot: f32, scale: vec2, sprite: &Sprite) {
        return Self::draw_texture_adv(pos, rot, scale, [RGBA32::WHITE; 4], sprite.handle().clone(), sprite.uv_rect());
    }

    pub fn draw_texture_adv(pos: vec2, rot: f32, scale: vec2, tint: [RGBA32; 4], texture: TextureHandle, uv_rect: Rect) {
        let Ok(mut graphics) = GRAPHICS.write() else { crash!("Couldn't access Graphics singleton!") };

        let extents = vec2::from(texture.dims()).scale(scale).scale(uv_rect.size()) / graphics.active_scope.pixels_per_unit();
        graphics.active_scope.draw_rect(RectSubmitCmd { pos, rot, extents, tint, texture, uv_rect });
    }

    pub fn draw_points(points: &[(vec2, RGBA32)]) { 
        let Ok(mut graphics) = GRAPHICS.write() else { crash!("Couldn't access Graphics singleton!") };
        graphics.active_scope.draw_points(PointsSubmitCmd { points });
    }

    pub fn draw_line(from: vec2, to: vec2, colors: [RGBA32; 2]) { 
        let Ok(mut graphics) = GRAPHICS.write() else { crash!("Couldn't access Graphics singleton!") };
        graphics.active_scope.draw_line(LineSubmitCmd { verts: [from, to], cols: colors });
    }

    pub fn draw_text(origin: vec2, rot: f32, extents: vec2, text: &str, font: &dyn Font) {
        let Ok(mut graphics) = GRAPHICS.write() else { crash!("Couldn't access Graphics singleton!") };
        graphics.active_scope.draw_text(origin, rot, extents, text, font);
    }

    pub fn draw_text_stateless(cfg: TextCfg, text: &str) {
        let Ok(mut graphics) = GRAPHICS.write() else { crash!("Couldn't access Graphics singleton!") };
        graphics.active_scope.draw_text_stateless(cfg, text);
    }

    /// Returns the current camera data.
    pub fn camera() -> CameraData {
        let Ok(graphics) = GRAPHICS.read() else { crash!("Couldn't access Graphics singleton!") };
        return graphics.active_scope.camera();
    }

    /// Returns the pixels per unit for textures.
    pub fn pixels_per_unit() -> f32 {
        let Ok(graphics) = GRAPHICS.read() else { crash!("Couldn't access Graphics singleton!") };
        return graphics.active_scope.pixels_per_unit();
    }

    /// Sets the pixels per unit for textures. Will panic if `ppu <= 0.0`.
    pub fn set_pixels_per_unit(ppu: f32) {
        let Ok(mut graphics) = GRAPHICS.write() else { crash!("Couldn't access Graphics singleton!") };
        graphics.active_scope.set_pixels_per_unit(ppu);
    }

    /// Returns the user data.
    pub fn user_data() -> i32 {
        let Ok(graphics) = GRAPHICS.read() else { crash!("Couldn't access Graphics singleton!") };
        return graphics.active_scope.user_data();
    }

    /// Sets user data.
    pub fn set_user_data(user_data: i32) {
        let Ok(mut graphics) = GRAPHICS.write() else { crash!("Couldn't access Graphics singleton!") };
        graphics.active_scope.set_user_data(user_data);
    }

    /// Returns the current pivot.
    pub fn pivot() -> vec2 {
        let Ok(graphics) = GRAPHICS.read() else { crash!("Couldn't access Graphics singleton!") };
        return graphics.active_scope.pivot();
    }

    /// Sets the current pivot.
    pub fn set_pivot(pivot: vec2) {
        let Ok(mut graphics) = GRAPHICS.write() else { crash!("Couldn't access Graphics singleton!") };
        graphics.active_scope.set_pivot(pivot);
    }

    /// Returns the active blending mode.
    pub fn blending_mode() -> BlendingMode {
        let Ok(graphics) = GRAPHICS.read() else { crash!("Couldn't access Graphics singleton!") };
        return graphics.active_scope.blending_mode();
    }

    /// Sets the active blending mode.
    pub fn set_blending_mode(blending: BlendingMode) {
        let Ok(mut graphics) = GRAPHICS.write() else { crash!("Couldn't access Graphics singleton!") };
        graphics.active_scope.set_blending_mode(blending);
    }

    /// Sets the active material.
    pub fn set_material(material: Arc<Material>) {
        let Ok(mut graphics) = GRAPHICS.write() else { crash!("Couldn't access Graphics singleton!") };
        graphics.active_scope.set_material(material);
    }

    /// Resets the active material.
    pub fn reset_material() {
        let Ok(mut graphics) = GRAPHICS.write() else { crash!("Couldn't access Graphics singleton!") };
        graphics.active_scope.reset_material();
    }

    /// Returns the active material.
    pub fn material() -> Arc<Material> {
        let Ok(graphics) = GRAPHICS.read() else { crash!("Couldn't access Graphics singleton!") };
        return graphics.active_scope.material();
    }

    /// Returns the font size.
    pub fn font_size() -> f32 {
        let Ok(graphics) = GRAPHICS.read() else { crash!("Couldn't access Graphics singleton!") };
        return graphics.active_scope.font_size();
    }

    /// Sets the font size.
    pub fn set_font_size(font_size: f32) {
        let Ok(mut graphics) = GRAPHICS.write() else { crash!("Couldn't access Graphics singleton!") };
        graphics.active_scope.set_font_size(font_size);
    }

    /// Returns the horizontal alignment for text.
    pub fn text_hor_alignment() -> HorTextAlign {
        let Ok(graphics) = GRAPHICS.read() else { crash!("Couldn't access Graphics singleton!") };
        return graphics.active_scope.text_hor_alignment();
    }

    /// Sets the horizontal alignment for text.
    pub fn set_text_hor_alignment(text_hor_alignment: HorTextAlign) {
        let Ok(mut graphics) = GRAPHICS.write() else { crash!("Couldn't access Graphics singleton!") };
        graphics.active_scope.set_text_hor_alignment(text_hor_alignment);
    }

    /// Returns the vertical alignment for text.
    pub fn text_ver_alignment() -> VerTextAlign {
        let Ok(graphics) = GRAPHICS.read() else { crash!("Couldn't access Graphics singleton!") };
        return graphics.active_scope.text_ver_alignment();
    }

    /// Sets the vertical alignment for text.
    pub fn set_text_ver_alignment(text_ver_alignment: VerTextAlign) {
        let Ok(mut graphics) = GRAPHICS.write() else { crash!("Couldn't access Graphics singleton!") };
        graphics.active_scope.set_text_ver_alignment(text_ver_alignment);
    }

    /// Returns the word wrap flag.
    pub fn word_wrap() -> bool {
        let Ok(graphics) = GRAPHICS.read() else { crash!("Couldn't access Graphics singleton!") };
        return graphics.active_scope.word_wrap();
    }

    /// Sets the word wrap flag.
    pub fn set_word_wrap(word_wrap: bool) {
        let Ok(mut graphics) = GRAPHICS.write() else { crash!("Couldn't access Graphics singleton!") };
        graphics.active_scope.set_word_wrap(word_wrap);
    }

    /// Returns the rich text flag.
    pub fn rich_text() -> bool {
        let Ok(graphics) = GRAPHICS.read() else { crash!("Couldn't access Graphics singleton!") };
        return graphics.active_scope.rich_text();
    }

    /// Sets the rich text flag.
    pub fn set_rich_text(rich_text: bool) {
        let Ok(mut graphics) = GRAPHICS.write() else { crash!("Couldn't access Graphics singleton!") };
        graphics.active_scope.set_rich_text(rich_text);
    }

    /// Returns the current configuration.
    pub fn cfg() -> RenderScopeCfgFlags {
        let Ok(graphics) = GRAPHICS.read() else { crash!("Couldn't access Graphics singleton!") };
        return graphics.active_scope.cfg();
    }

    /// Enables the configurations in `flags`.
    pub fn enable_cfg(flags: RenderScopeCfgFlags) {
        let Ok(mut graphics) = GRAPHICS.write() else { crash!("Couldn't access Graphics singleton!") };
        graphics.active_scope.enable_cfg(flags);
    }

    /// Disables the configurations in `flags`.
    pub fn disable_cfg(flags: RenderScopeCfgFlags) {
        let Ok(mut graphics) = GRAPHICS.write() else { crash!("Couldn't access Graphics singleton!") };
        graphics.active_scope.disable_cfg(flags);
    }

    /// Sets the configuration.
    pub fn set_cfg(flags: RenderScopeCfgFlags) {
        let Ok(mut graphics) = GRAPHICS.write() else { crash!("Couldn't access Graphics singleton!") };
        graphics.active_scope.set_cfg(flags);
    }

    /// Returns the target resolution.
    pub fn target_res() -> uvec2 {
        let Ok(graphics) = GRAPHICS.read() else { crash!("Couldn't access Graphics singleton!") };
        return graphics.active_scope.target_res();
    }


    pub(crate) fn init() {
        _ = WHITE_TEX.get(); // Initialize WHITE_TEX because why not
    }

    pub(crate) fn begin_render(camera: CameraData, target_res: uvec2, ui_res: Option<uvec2>, clear_col: RGBA32, pipeline: *const dyn RenderPipeline) {
        let Ok(mut graphics) = GRAPHICS.write() else { crash!("Couldn't access Graphics singleton!") };
        
        graphics.active_scope.begin_render(camera, target_res, clear_col, pipeline);
        if let Some(ui_res) = ui_res {
            graphics.ui_enabled = true;
            graphics.ui_scope.begin_render(ui_res, pipeline);
        } else {
            graphics.ui_enabled = false;
        }
    }

    pub(crate) fn end_render(real_window_res: uvec2) -> RenderStats { 
        let Ok(mut graphics) = GRAPHICS.write() else { crash!("Couldn't access Graphics singleton!") };

        // Swap with decoys so the borrow checker shuts the fuck up.
        let mut decoy_scope = RenderScope::new();
        let mut decoy_ui_scope = UIScope::new();
        std::mem::swap(&mut decoy_scope, &mut graphics.active_scope);
        std::mem::swap(&mut decoy_ui_scope, &mut graphics.ui_scope);

        let ui_data = if graphics.ui_enabled {
            Some(decoy_ui_scope.get_scene_data())
        } else {
            None
        };
        
        let stats = decoy_scope.end_render(&RenderTexture::to_screen(real_window_res), false, ui_data);

        // Swap them back so nobody notices
        std::mem::swap(&mut decoy_scope, &mut graphics.active_scope);
        std::mem::swap(&mut decoy_ui_scope, &mut graphics.ui_scope);

        return stats;
    }

    pub(crate) fn swap_scope(scope: &mut RenderScope) {
        let Ok(mut graphics) = GRAPHICS.write() else { crash!("Couldn't access Graphics singleton!") };
        std::mem::swap(scope, &mut graphics.active_scope);
    }
}


/// Represents the camera in Unit Space.
#[derive(Debug, Clone, PartialEq)]
pub struct CameraData {
    pub center: vec2,
    pub extents: vec2,
}

impl Default for CameraData {
    fn default() -> Self {
        Self { center: vec2::ZERO, extents: vec2::ONE }
    }
}


/// Holds all the required information to start a frame.
pub struct FrameSetup<'a> {
    /// Camera for regular rendering.
    pub camera: CameraData,
    
    /// Target resolution for regular rendering.
    pub target_res: uvec2,
    
    /// Target resolution for UI rendering. Set to `None` to disable UI rendering.
    pub ui_res: Option<uvec2>,

    /// Clear color for regular rendering. This is the color that will be placed in the background.
    pub clear_col: RGBA32,

    /// Optional custom render pipeline.
    pub pipeline: Option<&'a dyn RenderPipeline>,
}

impl<'a> Default for FrameSetup<'a> {
    fn default() -> Self {
        Self { camera: CameraData::default(), target_res: uvec2::ONE, ui_res: None, clear_col: RGBA32::BLACK, pipeline: None }
    }
}
