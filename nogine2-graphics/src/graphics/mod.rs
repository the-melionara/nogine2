use std::{sync::RwLock, thread::ThreadId};

use defaults::DefaultShaders;
use nogine2_core::{assert_expr, crash, log_error, math::{mat3x3::mat3, vector2::vec2, vector3::vec3}};
use vertex::BatchVertex;

use crate::{colors::{rgba::RGBA32, Color}, gl_wrapper::{buffer::{GlBuffer, GlBufferTarget, GlBufferUsage}, gl_render_elements, gl_uniform, gl_uniform_loc, to_byte_slice, vao::GlVertexArray}};

pub mod vertex;
pub mod defaults;
pub mod shader;


static GRAPHICS: RwLock<Graphics> = RwLock::new(Graphics::new());

pub struct Graphics {
    view_mat: mat3,

    render_started: bool,
    thread: Option<ThreadId>,
}

macro_rules! assert_main_thread {
    ($val:expr) => {
        nogine2_core::assert_expr!($val.thread == Some(std::thread::current().id()), "You can only call this function from the main thread after initializing the window!");
    };
}

impl Graphics {
    const fn new() -> Self {
        Self {
            view_mat: mat3::IDENTITY,

            render_started: false,
            thread: None,
        }
    }

    pub fn draw_rect(pos: vec2, rot: f32, extents: vec2, color: RGBA32) {
        let Ok(graphics) = GRAPHICS.read() else { crash!("Couldn't access Graphics singleton!") };
        assert_main_thread!(graphics);
        assert_expr!(graphics.render_started, "Render commands can only be called after Window::pre_tick!");

        let tf_mat = mat3::tf_matrix(pos, rot, extents.scale(vec2(1.0, -1.0)));

        let vbo = GlBuffer::new(GlBufferTarget::GlArrayBuffer, to_byte_slice(&[
                BatchVertex { pos: (&tf_mat * vec3(0.0, 0.0, 1.0)).xy(), tint: color, uv: vec2::UP,    tex_id: 0, user_data: 0 },
                BatchVertex { pos: (&tf_mat * vec3(0.0, 1.0, 1.0)).xy(), tint: color, uv: vec2::ZERO,  tex_id: 0, user_data: 0 },
                BatchVertex { pos: (&tf_mat * vec3(1.0, 1.0, 1.0)).xy(), tint: color, uv: vec2::RIGHT, tex_id: 0, user_data: 0 },
                BatchVertex { pos: (&tf_mat * vec3(1.0, 0.0, 1.0)).xy(), tint: color, uv: vec2::ONE,   tex_id: 0, user_data: 0 },
        ]), GlBufferUsage::StaticDraw);
        let ebo = GlBuffer::new(GlBufferTarget::GlElementArrayBuffer, to_byte_slice(&[0u16, 1, 2, 2, 3, 0]), GlBufferUsage::StaticDraw);
    
        let mut vao = GlVertexArray::new();
        vao.bind_vbo(&vbo, BatchVertex::VERT_ATTRIB_DEFINITIONS);
        ebo.bind();
    
        let shader = DefaultShaders::batch();
        if !shader.use_shader() {
            log_error!("GL_ERROR: Couldn't render!");
            return;
        }
    
        if let Some(view_mat_loc) = gl_uniform_loc(shader.gl_obj(), b"uViewMat\0") {
            gl_uniform::set_mat3(view_mat_loc, &Graphics::view_mat());
        }
    
        gl_render_elements(6);
    }


    pub(crate) fn init() {
        let Ok(mut graphics) = GRAPHICS.write() else { crash!("Couldn't access Graphics singleton!") };

        graphics.thread = Some(std::thread::current().id());
    }

    pub(crate) fn begin_render(cam_data: CameraData) { 
        let Ok(mut graphics) = GRAPHICS.write() else { crash!("Couldn't access Graphics singleton!") };
        assert_main_thread!(graphics);

        graphics.view_mat = mat3::tf_matrix(cam_data.center, 0.0, cam_data.extents.scale(vec2(1.0, -1.0)) * 0.5).inverse().unwrap_or(mat3::IDENTITY);
        graphics.render_started = true;
    }

    pub(crate) fn end_render() { 
        let Ok(mut graphics) = GRAPHICS.write() else { crash!("Couldn't access Graphics singleton!") };
        assert_main_thread!(graphics);

        graphics.render_started = false;
    }

    pub(crate) fn view_mat() -> mat3 {
        let Ok(graphics) = GRAPHICS.read() else { crash!("Couldn't access Graphics singleton!") };
        assert_main_thread!(graphics);

        return graphics.view_mat.clone();
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
