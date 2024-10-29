use std::{sync::RwLock, thread::ThreadId};

use nogine2_core::{crash, math::vector2::vec2};

pub mod vertex;


static GRAPHICS: RwLock<Graphics> = RwLock::new(Graphics::new());

pub struct Graphics {
    thread: Option<ThreadId>,
}

macro_rules! assert_main_thread {
    ($val:expr) => {
        assert_expr!($val.thread == Some(std::thread::current().id()), "You can only call this function from the main thread after initializing the window!");
    };
}

impl Graphics {
    const fn new() -> Self {
        Self { thread: None }
    }

    pub(crate) fn init() {
        let Ok(mut graphics) = GRAPHICS.write() else { crash!("Couldn't access Graphic's singleton!") };

        graphics.thread = Some(std::thread::current().id());
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
