use glfw::{glfwInit, glfwSetErrorCallback, glfwTerminate, GLFWbool};
use nogine2_core::{crash, log_info};
use window::{Window, POST_TICK_EVS, PRE_TICK_EVS};

pub mod window;
pub mod input;

mod glfw;

/// Logger must be initialized before
fn init_glfw() {
    unsafe {
        if glfwInit() != GLFWbool::TRUE {
            crash!("Couldn't initialize GLFW!");
        }

        glfwSetErrorCallback(glfw_callbacks::error_callback);
        log_info!("NOGINE2: GLFW initialized")
    }    
}

fn deinit_glfw() {
    unsafe { glfwTerminate() };
}


pub fn window_subscribe_pre_tick(f: fn(&Window)) {
    PRE_TICK_EVS.write().unwrap().subscribe(f);
}

pub fn window_subscribe_post_tick(f: fn(&Window)) {
    POST_TICK_EVS.write().unwrap().subscribe(f);
}


mod glfw_callbacks {
    use std::ffi::{c_char, c_double, c_int, CStr};

    use nogine2_core::{log_error, math::vector2::vec2};

    use crate::{glfw::{GLFWaction, GLFWkey, GLFWmousebutton, GLFWwindow}, input::Input};

    pub extern "C" fn error_callback(error: c_int, description: *const c_char) {
        if let Ok(msg) = unsafe { CStr::from_ptr(description).to_str() } {
            log_error!("GLFW Error {error}: {msg}");
        } else {
            log_error!("GLFW Error {error}: Unparseable C error");
        }
    }

    pub extern "C" fn key_callback(_: *mut GLFWwindow, key: GLFWkey, _: c_int, action: GLFWaction, _: c_int) {
        Input::submit_key(key, action);
    }

    pub extern "C" fn cursor_pos_callback(_: *mut GLFWwindow, xpos: c_double, ypos: c_double) {
        Input::submit_mouse_pos(vec2(xpos as f32, ypos as f32));
    }

    pub extern "C" fn mouse_sroll_callback(_: *mut GLFWwindow, xoffset: c_double, yoffset: c_double) {
        Input::submit_mouse_scroll(vec2(xoffset as f32, yoffset as f32));
    }

    pub extern "C" fn mouse_button_callback(_: *mut GLFWwindow, button: GLFWmousebutton, action: GLFWaction, _: c_int) {
        Input::submit_mouse_button(button, action);
    }
}
