use std::{sync::RwLock, thread::ThreadId};

use nogine2_core::{crash, math::{mat3x3::mat3, vector2::vec2}};

pub mod vertex;
pub mod defaults;
pub mod shader;


static GRAPHICS: RwLock<Graphics> = RwLock::new(Graphics::new());

pub struct Graphics {
    view_mat: mat3,

    thread: Option<ThreadId>,
}

macro_rules! assert_main_thread {
    ($val:expr) => {
        nogine2_core::assert_expr!($val.thread == Some(std::thread::current().id()), "You can only call this function from the main thread after initializing the window!");
    };
}

impl Graphics {
    const fn new() -> Self {
        Self { thread: None, view_mat: mat3::IDENTITY }
    }

    pub(crate) fn init() {
        let Ok(mut graphics) = GRAPHICS.write() else { crash!("Couldn't access Graphics singleton!") };

        graphics.thread = Some(std::thread::current().id());
    }

    pub(crate) fn begin_render(cam_data: CameraData) { 
        let Ok(mut graphics) = GRAPHICS.write() else { crash!("Couldn't access Graphics singleton!") };
        assert_main_thread!(graphics);

        graphics.view_mat = mat3::tf_matrix(cam_data.center, 0.0, cam_data.extents.scale(vec2(1.0, -1.0)) * 0.5).inverse().unwrap_or(mat3::IDENTITY);
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
