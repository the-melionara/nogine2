use std::ffi::c_void;

mod gl;

pub fn gl_load(f: impl Fn(&str) -> *const c_void) {
    gl::load_with(f);
}
