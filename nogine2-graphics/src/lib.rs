use std::ffi::c_void;

use colors::rgba::RGBA32;
use gl_wrapper::{gl_enable_blend, gl_load};
use graphics::{defaults::{DefaultShaders, DefaultSubShaders}, pipeline::{RenderPipeline, RenderStats}, CameraData, Graphics};
use nogine2_core::{log_info, math::vector2::uvec2};

pub mod graphics;
pub mod colors;

mod gl_wrapper;

pub fn init_graphics(load_fn: impl Fn(&str) -> *const c_void) -> bool {
    gl_load(load_fn);
    gl_enable_blend();

    if !DefaultSubShaders::init() { return false };
    if !DefaultShaders::init() { return false };
    Graphics::init();

    log_info!("NOGINE2: Graphics initialized");
    return true;
}

pub fn global_begin_render(camera: CameraData, target_res: uvec2, clear_col: RGBA32, pipeline: *const dyn RenderPipeline) {
    Graphics::begin_render(camera, target_res, clear_col, pipeline);
}

pub fn global_end_render(real_window_res: uvec2) -> RenderStats {
    return Graphics::end_render(real_window_res);
}
