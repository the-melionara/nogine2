use std::ffi::c_void;

use nogine2_core::log_info;

pub mod graphics;

mod gl_wrapper;

pub fn init_graphics(load_fn: impl Fn(&str) -> *const c_void) {
    gl_wrapper::gl_load(load_fn);

    log_info!("NOGINE2: Graphics initialized");
}
