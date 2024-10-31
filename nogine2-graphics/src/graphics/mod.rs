use std::{sync::RwLock, thread::ThreadId};

use batch::{BatchData, BatchPushCmd};
use nogine2_core::{assert_expr, crash, math::{mat3x3::mat3, vector2::{uvec2, vec2}, vector3::vec3}};
use pipeline::RenderStats;
use vertex::BatchVertex;

use crate::colors::rgba::RGBA32;

pub mod vertex;
pub mod defaults;
pub mod shader;
pub mod pipeline;
pub mod texture;

mod batch;

static GRAPHICS: RwLock<Graphics> = RwLock::new(Graphics::new());

pub struct Graphics {
    batch_data: BatchData,

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
            batch_data: BatchData::new(),

            render_started: false,
            thread: None,
        }
    }

    pub fn draw_rect(pos: vec2, rot: f32, extents: vec2, color: RGBA32) {
        let Ok(mut graphics) = GRAPHICS.write() else { crash!("Couldn't access Graphics singleton!") };
        assert_main_thread!(graphics);
        assert_expr!(graphics.render_started, "Render commands can only be called after Window::pre_tick!");

        let tf_mat = mat3::tf_matrix(pos, rot, extents.scale(vec2(1.0, -1.0)));

        let verts = &[
            BatchVertex { pos: (&tf_mat * vec3(0.0, 0.0, 1.0)).xy(), tint: color, uv: vec2::UP,    tex_id: 0, user_data: 0 },
            BatchVertex { pos: (&tf_mat * vec3(0.0, 1.0, 1.0)).xy(), tint: color, uv: vec2::ZERO,  tex_id: 0, user_data: 0 },
            BatchVertex { pos: (&tf_mat * vec3(1.0, 1.0, 1.0)).xy(), tint: color, uv: vec2::RIGHT, tex_id: 0, user_data: 0 },
            BatchVertex { pos: (&tf_mat * vec3(1.0, 0.0, 1.0)).xy(), tint: color, uv: vec2::ONE,   tex_id: 0, user_data: 0 },
        ];
        let indices = &[0, 1, 2, 2, 3, 0];
    
        graphics.batch_data.push(BatchPushCmd { verts, indices });
    }

    /// Returns the current camera data.
    pub fn camera() -> CameraData {
        let Ok(graphics) = GRAPHICS.read() else { crash!("Couldn't access Graphics singleton!") };
        assert_main_thread!(graphics);

        return graphics.batch_data.camera();
    }

    pub(crate) fn init() {
        let Ok(mut graphics) = GRAPHICS.write() else { crash!("Couldn't access Graphics singleton!") };

        graphics.thread = Some(std::thread::current().id());
    }

    pub(crate) fn begin_render(camera: CameraData, target_res: uvec2) { 
        let Ok(mut graphics) = GRAPHICS.write() else { crash!("Couldn't access Graphics singleton!") };
        assert_main_thread!(graphics);

        graphics.batch_data.setup_frame(camera, target_res);
        graphics.render_started = true;
    }

    pub(crate) fn end_render() -> RenderStats { 
        let Ok(mut graphics) = GRAPHICS.write() else { crash!("Couldn't access Graphics singleton!") };
        assert_main_thread!(graphics);

        graphics.render_started = false;
        let batch_stats = graphics.batch_data.render();

        return RenderStats { batch: batch_stats };
    }

    pub(crate) fn assert_main_thread() {
        let Ok(graphics) = GRAPHICS.read() else { crash!("Couldn't access Graphics singleton!") };
        assert_main_thread!(graphics);
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
