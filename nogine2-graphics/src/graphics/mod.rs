use std::{sync::RwLock, thread::ThreadId};

use nogine2_core::{crash, math::vector2::{uvec2, vec2}};

use crate::colors::rgba::RGBA32;

static GRAPHICS: RwLock<Graphics> = RwLock::new(Graphics::new());

pub struct Graphics {
    thread: Option<ThreadId>,
}

macro_rules! assert_main_thread {
    ($val:expr) => {
        assert_expr!($val.thread == Some(std::thread::current().id()), "You can only call this function from the main thread!");
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

/// Represents the required data to begin rendering.
pub struct BeginRenderCmd<'a> {
    pub camera: CameraData,
    pub target_res: uvec2,
    pub pipeline: &'a (),
    pub clear_col: RGBA32,
}

static TEMP_PIPELINE: () = ();
impl<'a> BeginRenderCmd<'a> {
    /// Creates a command with the default pipeline.
    pub fn new(camera: CameraData, target_res: uvec2, clear_col: RGBA32) -> Self {
        Self { camera, target_res, pipeline: &TEMP_PIPELINE, clear_col }
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
