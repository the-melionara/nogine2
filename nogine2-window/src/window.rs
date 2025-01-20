use std::{ffi::CString, sync::{atomic::{AtomicBool, Ordering}, RwLock}, thread::ThreadId, time::{Duration, Instant}};

use nogine2_core::{assert_expr, crash, event::Event, log_info, math::vector2::{ivec2, uvec2, vec2}};
use nogine2_graphics::{global_begin_render, global_end_render, graphics::{pipeline::{DefaultPipeline, RenderPipeline, RenderStats}, FrameSetup}, init_graphics};

use crate::{deinit_glfw, glfw::{glfwCreateWindow, glfwDestroyWindow, glfwGetFramebufferSize, glfwGetPrimaryMonitor, glfwGetProcAddress, glfwGetVideoMode, glfwGetWindowMonitor, glfwGetWindowSize, glfwIconifyWindow, glfwMakeContextCurrent, glfwMaximizeWindow, glfwPollEvents, glfwRequestWindowAttention, glfwRestoreWindow, glfwSetCursorPosCallback, glfwSetKeyCallback, glfwSetMouseButtonCallback, glfwSetScrollCallback, glfwSetWindowMonitor, glfwSetWindowSize, glfwSetWindowTitle, glfwSwapBuffers, glfwSwapInterval, glfwWindowShouldClose, GLFWbool, GLFWwindow}, glfw_callbacks, init_glfw, input::Input};

#[derive(Debug, Clone)]
pub struct WindowCfg<'a> {
    pub title: &'a str,
    pub res: uvec2,
}

static DEFAULT_PIPELINE: DefaultPipeline = DefaultPipeline;

static MAIN_WINDOW_EXISTS: AtomicBool = AtomicBool::new(false);

macro_rules! assert_main_thread {
    ($val:expr) => {
        assert_expr!($val.thread == std::thread::current().id(), "You can only call this function from the main thread!");
    };
}

pub(crate) static PRE_TICK_EVS: RwLock<Event<Window>> = RwLock::new(Event::new());
pub(crate) static POST_TICK_EVS: RwLock<Event<Window>> = RwLock::new(Event::new());

pub struct Window {
    glfw_window: *mut GLFWwindow,
    title: String,
    best_res: uvec2,

    ts: f32,
    last_frame: Instant,
    first_frame: Instant,

    thread: ThreadId,
}

impl Window {
    /// Creates the `Window` object and initializes `nogine2`. Will panic if a window already exists, if `cfg.title` contains any '\0' or couldn't create the window or initialize the OpenGL context.
    pub fn new(cfg: WindowCfg<'_>) -> Self {
        if MAIN_WINDOW_EXISTS.fetch_or(true, Ordering::AcqRel) {
            crash!("A main window already exists!");
        }
        
        init_glfw();

        unsafe {
            let Ok(title) = CString::new(cfg.title) else {
                crash!("The title of a window must not have any \\0 inside its body!");
            };
            let window = glfwCreateWindow(cfg.res.0 as i32, cfg.res.1 as i32, title.as_ptr(), std::ptr::null_mut(), std::ptr::null_mut());
            if window.is_null() {
                crash!("Couldn't create Window or initialize an OpenGL context!");
            }
            glfwMakeContextCurrent(window);
            glfwSetKeyCallback(window, glfw_callbacks::key_callback);
            glfwSetCursorPosCallback(window, glfw_callbacks::cursor_pos_callback);
            glfwSetScrollCallback(window, glfw_callbacks::mouse_sroll_callback);
            glfwSetMouseButtonCallback(window, glfw_callbacks::mouse_button_callback);

            if !init_graphics(|x| {
                let cstring = CString::new(x).unwrap();
                glfwGetProcAddress(cstring.as_ptr())
            }) {
                crash!("Couldn't initialize graphics!");
            }

            log_info!("NOGINE2: Window created");
            return Self {
                glfw_window: window,
                title: cfg.title.to_string(),best_res: cfg.res,
                ts: 0.02, first_frame: Instant::now(), last_frame: Instant::now(),
                thread: std::thread::current().id()
            };
        }
    }

    /// Returns if the `Window` is open.
    pub fn is_open(&self) -> bool {
        return unsafe { glfwWindowShouldClose(self.glfw_window) } == GLFWbool::FALSE;
    }

    /// Executes at the start of every frame.
    pub fn pre_tick<'a>(&'a mut self, setup: FrameSetup<'a>) {
        assert_main_thread!(self);

        let pipeline = if let Some(pipeline) = setup.pipeline {
            unsafe { std::mem::transmute::<_, *const dyn RenderPipeline>(pipeline) } // Hack to stop misdiagnosis from rust (?)
        } else {
            &DEFAULT_PIPELINE as *const dyn RenderPipeline
        };

        global_begin_render(setup.camera, setup.target_res, setup.ui_res, setup.clear_col, pipeline);
        PRE_TICK_EVS.read().unwrap().call(self);
    }

    /// Executes at the end of every frame.
    pub fn post_tick(&mut self) -> RenderStats {
        assert_main_thread!(self);

        Input::flush();
        let render_stats = global_end_render(self.fb_size());
        unsafe {
            glfwSwapBuffers(self.glfw_window);
            glfwPollEvents();
        }

        POST_TICK_EVS.read().unwrap().call(self);

        self.ts = self.last_frame.elapsed().as_secs_f32();
        self.last_frame = Instant::now();

        // Update best_res
        if !self.fullscreen() {
            self.best_res = self.res();
        }

        return render_stats;
    }

    /// Sets vsync.
    pub fn set_vsync(&mut self, enabled: bool) {
        assert_main_thread!(self);
        unsafe { glfwSwapInterval(if enabled { 1 } else { 0 }) };
    }

    /// Returns the elapsed time since the last frame in seconds.
    pub fn ts(&self) -> f32 {
        self.ts
    }

    /// Returns the elapsed time since the first frame.
    pub fn time(&self) -> Duration {
        self.first_frame.elapsed()
    }

    /// Returns the window's resolution in pixels. **NOTE:** In some platforms this resolution does not directly represent the framebuffer's size (if you need that, use `.fb_size()`).
    pub fn res(&self) -> uvec2 {
        let mut res = ivec2::ZERO;
        unsafe { glfwGetWindowSize(self.glfw_window, &mut res.0, &mut res.1) };
        return uvec2::from(res);
    }

    /// Sets the window's resolution. **NOTE:** In some platforms this resolution does not directly represent the framebuffer's size (there's no consistent way to directly change it, so just keep that in mind).
    pub fn set_res(&mut self, res: uvec2) {
        assert_main_thread!(self);
        unsafe { glfwSetWindowSize(self.glfw_window, res.0 as i32, res.1 as i32) };
        self.best_res = res;
    }

    /// Returns the window's framebuffer size.
    pub fn fb_size(&self) -> uvec2 {
        let mut res = ivec2::ZERO;
        unsafe { glfwGetFramebufferSize(self.glfw_window, &mut res.0, &mut res.1) };
        return uvec2::from(res);
    }

    /// Returns the aspect ratio of the window.
    pub fn aspect_ratio(&self) -> f32 {
        let res = vec2::from(self.res());
        return res.0 / res.1;
    }

    /// Returns the window's title.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Sets the window's title.
    pub fn set_title(&mut self, title: impl Into<String>) {
        assert_main_thread!(self);

        let title = title.into();
        let Ok(c_title) = CString::new(title.as_str()) else {
            crash!("The title of a window must not have any \\0 inside its body!");
        };

        unsafe { glfwSetWindowTitle(self.glfw_window, c_title.as_ptr()) };
        self.title = title;
    }

    /// Minimizes the window.
    pub fn minimize(&mut self) {
        assert_main_thread!(self);
        unsafe { glfwIconifyWindow(self.glfw_window) };
    }

    /// Restores the window.
    pub fn restore(&mut self) {
        assert_main_thread!(self);
        unsafe { glfwRestoreWindow(self.glfw_window) };
    }

    /// Maximizes the window.
    pub fn maximize(&mut self) {
        assert_main_thread!(self);
        unsafe { glfwMaximizeWindow(self.glfw_window) };
    }

    /// Requests attention to the window.
    pub fn request_attention(&mut self) {
        assert_main_thread!(self);
        unsafe { glfwRequestWindowAttention(self.glfw_window) };
    }

    /// Returns if the window is full screen.
    pub fn fullscreen(&self) -> bool {
        assert_main_thread!(self);
        return !unsafe { glfwGetWindowMonitor(self.glfw_window) }.is_null();
    }

    /// Sets a window to be fullscreen.
    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        assert_main_thread!(self);
        unsafe {
            let monitor = glfwGetPrimaryMonitor();
            let vid_mode = glfwGetVideoMode(monitor).as_ref().unwrap_unchecked();

            if fullscreen {
                glfwSetWindowMonitor(self.glfw_window, monitor, 0, 0, vid_mode.width, vid_mode.height, vid_mode.refreshRate);
            } else {
                glfwSetWindowMonitor(
                    self.glfw_window, std::ptr::null_mut(),
                    (vid_mode.width - self.best_res.0 as i32) / 2, (vid_mode.height - self.best_res.1 as i32) / 2,
                    self.best_res.0 as i32, self.best_res.1 as i32,
                    0
                );
            }
        }
    }

    /// Toggles fullscreen.
    pub fn toggle_fullscreen(&mut self) {
        self.set_fullscreen(!self.fullscreen());
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        assert_main_thread!(self);

        // Don't change MAIN_WINDOW_EXISTS to avoid multiple nogine2 initializations
        unsafe { glfwDestroyWindow(self.glfw_window) };
        deinit_glfw();
    }
}
