use std::ffi::c_void;

use gl_wrapper::{gl_enable_blend, gl_load};
use graphics::Graphics;
use nogine2_core::log_info;

pub mod graphics;
pub mod colors;

mod gl_wrapper;

pub fn init_graphics(load_fn: impl Fn(&str) -> *const c_void) {
    gl_load(load_fn);
    gl_enable_blend();

    Graphics::init();

    log_info!("NOGINE2: Graphics initialized");
}
