use std::ffi::c_void;

use colors::rgba::RGBA32;
use gl_wrapper::{gl_clear, gl_enable_blend, gl_load, gl_viewport};
use graphics::{defaults::{DefaultShaders, DefaultSubShaders}, CameraData, Graphics};
use nogine2_core::{log_info, math::{rect::IRect, vector2::{ivec2, uvec2}}};

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

pub fn global_begin_render(camera: CameraData, target_res: uvec2, clear_col: RGBA32, pipeline: *const ()) {
    gl_clear(clear_col);
    gl_viewport(IRect { start: ivec2::ZERO, end: target_res.into() });
    Graphics::begin_render(camera);
}

pub fn global_end_render() {
    Graphics::end_render();
}
