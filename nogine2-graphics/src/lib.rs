use std::{ffi::c_void, sync::Mutex};

use colors::rgba::RGBA32;
use gl_wrapper::{gl_enable_blend, gl_load};
use graphics::{defaults::{DefaultMaterials, DefaultShaders, DefaultSubShaders}, pipeline::{RenderPipeline, RenderStats}, CameraData, Graphics};
use nogine2_core::{log_info, math::vector2::uvec2};

pub mod graphics;
pub mod colors;

mod gl_wrapper;

pub fn init_graphics(load_fn: impl Fn(&str) -> *const c_void) -> bool {
    gl_load(load_fn);
    gl_enable_blend();

    if !DefaultSubShaders::init() { return false };
    if !DefaultShaders::init() { return false };
    if !DefaultMaterials::init() { return false };
    Graphics::init();

    log_info!("NOGINE2: Graphics initialized");
    return true;
}

pub(crate) static TIME_TS: Mutex<(f32, f32)> = Mutex::new((0.0, 0.0));

pub fn global_begin_render(
    camera: CameraData,
    target_res: uvec2,
    ui_res: Option<uvec2>,
    clear_col: RGBA32,
    pipeline: *const dyn RenderPipeline,
    time: f32,
    ts: f32,
) {
    {
        let mut time_ts = TIME_TS.lock().unwrap();
        *time_ts = (time, ts);
    }

    Graphics::begin_render(camera, target_res, ui_res, clear_col, pipeline);
}

pub fn global_end_render(real_window_res: uvec2) -> RenderStats {
    return Graphics::end_render(real_window_res);
}
