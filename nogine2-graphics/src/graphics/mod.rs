use std::{sync::{Arc, RwLock}, thread::ThreadId};

use blending::BlendingMode;
use material::Material;
use nogine2_core::{crash, math::{rect::Rect, vector2::{uvec2, vec2}}};
use pipeline::{RenderPipeline, RenderStats};
use scope::{LineSubmitCmd, PointsSubmitCmd, RectSubmitCmd, RenderScope, RenderScopeCfgFlags};
use texture::{pixels::{PixelFormat, Pixels}, rendertex::RenderTexture, sprite::Sprite, Texture2D, TextureFiltering, TextureHandle, TextureSampling, TextureWrapping};

use crate::colors::{rgba::RGBA32, Color};

pub mod vertex;
pub mod defaults;
pub mod shader;
pub mod pipeline;
pub mod texture;
pub mod blending;
pub mod scope;
pub mod gfx;
pub mod material;

mod batch;

static GRAPHICS: RwLock<Graphics> = RwLock::new(Graphics::new());

pub struct Graphics {
    active_scope: RenderScope,
    white_texture: Option<TextureHandle>,

    thread: Option<ThreadId>,
}

impl Graphics {
    const fn new() -> Self {
        Self {
            active_scope: RenderScope::new(),
            white_texture: None,

            thread: None,
        }
    }

    pub fn draw_rect(pos: vec2, rot: f32, extents: vec2, color: RGBA32) {
        let Ok(mut graphics) = GRAPHICS.write() else { crash!("Couldn't access Graphics singleton!") };

        let white_texture = graphics.white_texture.clone().unwrap();
        graphics.active_scope.draw_rect(RectSubmitCmd { pos, rot, extents, tint: [color; 4], texture: white_texture, uv_rect: Rect::IDENT });
    }

    pub fn draw_texture(pos: vec2, rot: f32, scale: vec2, tint: RGBA32, texture: &Texture2D) {
        let Ok(mut graphics) = GRAPHICS.write() else { crash!("Couldn't access Graphics singleton!") };

        let extents = vec2::from(texture.dims()).scale(scale) / graphics.active_scope.pixels_per_unit();
        graphics.active_scope.draw_rect(RectSubmitCmd { pos, rot, extents, tint: [tint; 4], texture: texture.handle(), uv_rect: Rect::IDENT });
    }

    pub fn draw_sprite(pos: vec2, rot: f32, scale: vec2, sprite: &Sprite) {
        let Ok(mut graphics) = GRAPHICS.write() else { crash!("Couldn't access Graphics singleton!") };

        let extents = vec2::from(sprite.dims()).scale(scale) / graphics.active_scope.pixels_per_unit();
        graphics.active_scope.draw_rect(RectSubmitCmd { pos, rot, extents, tint: [RGBA32::WHITE; 4], texture: sprite.handle().clone(), uv_rect: sprite.uv_rect() });
    }

    pub fn draw_points(points: &[(vec2, RGBA32)]) { 
        let Ok(mut graphics) = GRAPHICS.write() else { crash!("Couldn't access Graphics singleton!") };
        graphics.active_scope.draw_points(PointsSubmitCmd { points });
    }

    pub fn draw_line(from: vec2, to: vec2, colors: [RGBA32; 2]) { 
        let Ok(mut graphics) = GRAPHICS.write() else { crash!("Couldn't access Graphics singleton!") };
        graphics.active_scope.draw_lines(LineSubmitCmd { verts: [from, to], cols: colors });
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


    pub(crate) fn init() {
        let Ok(mut graphics) = GRAPHICS.write() else { crash!("Couldn't access Graphics singleton!") };

        graphics.thread = Some(std::thread::current().id());
        graphics.white_texture = Some(Texture2D::new(
            Pixels::new(vec![255, 255, 255, 255], uvec2(1, 1), PixelFormat::RGBA8),
            TextureSampling { filtering: TextureFiltering::Nearest, wrapping: TextureWrapping::Clamp },
        ).handle());
    }

    pub(crate) fn begin_render(camera: CameraData, target_res: uvec2, clear_col: RGBA32, pipeline: *const dyn RenderPipeline) {
        let Ok(mut graphics) = GRAPHICS.write() else { crash!("Couldn't access Graphics singleton!") };
        graphics.active_scope.begin_render(camera, target_res, clear_col, pipeline);
    }

    pub(crate) fn end_render(real_window_res: uvec2) -> RenderStats { 
        let Ok(mut graphics) = GRAPHICS.write() else { crash!("Couldn't access Graphics singleton!") };
        return graphics.active_scope.end_render(&RenderTexture::to_screen(real_window_res));
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



