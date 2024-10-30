use std::ffi::c_void;

use colors::{rgba::RGBA32, Color};
use gl_wrapper::{buffer::{GlBuffer, GlBufferTarget, GlBufferUsage}, gl_clear, gl_enable_blend, gl_load, gl_render_elements, gl_uniform, gl_uniform_loc, gl_viewport, to_byte_slice, vao::GlVertexArray};
use graphics::{defaults::{DefaultShaders, DefaultSubShaders}, vertex::BatchVertex, CameraData, Graphics};
use nogine2_core::{log_error, log_info, math::{rect::IRect, vector2::{ivec2, uvec2, vec2}}};

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
    let vbo = GlBuffer::new(GlBufferTarget::GlArrayBuffer, to_byte_slice(&[
            BatchVertex { pos: vec2(-0.5, -0.5), tint: RGBA32::RED, uv: vec2(0.0, 0.0), tex_id: 0, user_data: 0 },
            BatchVertex { pos: vec2( 0.0,  0.5), tint: RGBA32::GREEN, uv: vec2(0.5, 1.0), tex_id: 0, user_data: 0 },
            BatchVertex { pos: vec2( 0.5, -0.5), tint: RGBA32::BLUE, uv: vec2(1.0, 0.0), tex_id: 0, user_data: 0 },
    ]), GlBufferUsage::StaticDraw);
    let ebo = GlBuffer::new(GlBufferTarget::GlElementArrayBuffer, to_byte_slice(&[0u32, 1, 2]), GlBufferUsage::StaticDraw);

    let mut vao = GlVertexArray::new();
    vao.bind_vbo(&vbo, BatchVertex::VERT_ATTRIB_DEFINITIONS);
    ebo.bind();

    let batch = DefaultShaders::batch();
    if !batch.use_shader() {
        log_error!("GL_ERROR: Couldn't render!");
        return;
    }

    if let Some(view_mat_loc) = gl_uniform_loc(batch.gl_obj(), b"uViewMat\0") {
        gl_uniform::set_mat3(view_mat_loc, &Graphics::view_mat());
    }

    gl_render_elements(3);
}
