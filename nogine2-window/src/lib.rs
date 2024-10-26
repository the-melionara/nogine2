use glfw::{glfwInit, glfwSetErrorCallback, glfwTerminate, GLFWbool};
use nogine2_core::{crash, log_info};

pub mod window;

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


mod glfw_callbacks {
    use std::ffi::{c_char, c_int, CStr};

    use nogine2_core::log_error;

    pub extern "C" fn error_callback(error: c_int, description: *const c_char) {
        if let Ok(msg) = unsafe { CStr::from_ptr(description).to_str() } {
            log_error!("GLFW Error {error}: {msg}");
        } else {
            log_error!("GLFW Error {error}: Unparseable C error");
        }
    }
}
