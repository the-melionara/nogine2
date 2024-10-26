use std::{ffi::CString, sync::atomic::{AtomicBool, Ordering}, time::Instant};

use nogine2_core::{crash, log::init_log, log_info, math::vector2::uvec2};

use crate::{deinit_glfw, glfw::{glfwCreateWindow, glfwDestroyWindow, glfwMakeContextCurrent, glfwPollEvents, glfwSwapBuffers, glfwSwapInterval, glfwWindowShouldClose, GLFWbool, GLFWwindow}, init_glfw};

#[derive(Debug, Clone)]
pub struct WindowCfg<'a> {
    pub title: &'a str,
    pub res: uvec2,
}

static MAIN_WINDOW_EXISTS: AtomicBool = AtomicBool::new(false);

pub struct Window {
    glfw_window: *mut GLFWwindow,

    ts: f32,
    last_frame: Instant,
}

impl Window {
    /// Creates the `Window` object and initializes `nogine2`. Will panic if a window already exists, if `cfg.title` contains any '\0' or couldn't create the window or initialize the OpenGL context.
    pub fn new(cfg: WindowCfg<'_>) -> Self {
        if MAIN_WINDOW_EXISTS.fetch_or(true, Ordering::AcqRel) {
            crash!("A main window already exists!");
        }
        
        init_log();
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

            log_info!("NOGINE2: Window created");
            return Self { glfw_window: window, ts: 0.02, last_frame: Instant::now() };
        }
    }

    /// Returns if the `Window` is open.
    pub fn is_open(&self) -> bool {
        return unsafe { glfwWindowShouldClose(self.glfw_window) } == GLFWbool::FALSE;
    }

    /// Executes at the start of every frame.
    pub fn pre_tick(&mut self) {
        
    }

    /// Executes at the end of every frame.
    pub fn post_tick(&mut self) {
        unsafe {
            glfwSwapBuffers(self.glfw_window);
            glfwPollEvents();
        }

        self.ts = self.last_frame.elapsed().as_secs_f32();
        self.last_frame = Instant::now();
    }

    /// Sets vsync.
    pub fn set_vsync(&mut self, enabled: bool) {
       unsafe { glfwSwapInterval(if enabled { 1 } else { 0 }) };
    }

    /// Returns the elapsed time since the last frame in seconds.
    pub fn ts(&self) -> f32 {
        self.ts
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        // Don't change MAIN_WINDOW_EXISTS to avoid multiple nogine2 initializations
        unsafe { glfwDestroyWindow(self.glfw_window) };
        deinit_glfw();
    }
}
