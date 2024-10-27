#![allow(non_camel_case_types)]
#![allow(unused)]

use std::ffi::{c_char, c_double, c_int, c_void};

#[allow(dead_code)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GLFWbool {
    FALSE = 0,
    TRUE = 1,
}

impl From<GLFWbool> for bool {
    fn from(value: GLFWbool) -> Self {
        match value {
            GLFWbool::FALSE => false,
            GLFWbool::TRUE => true,
        }
    }
}

pub const GLFW_CLIENT_API: c_int = 0x00022001;

pub const GLFW_NO_API: c_int = 0;


// c_void in values because C ffi doesn't allow zero sized types
#[repr(C)] pub struct GLFWwindow(c_void);
#[repr(C)] pub struct GLFWmonitor(c_void);

#[allow(non_snake_case)]
#[repr(C)] pub struct GLFWvidmode {
    pub width: c_int,
    pub height: c_int,
    pub redBits: c_int,
    pub greenBits: c_int,
    pub blueBits: c_int,
    pub refreshRate: c_int,
}

#[repr(i32)]
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GLFWaction {
    RELEASE = 0,
    PRESS = 1,
    REPEAT = 2,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GLFWkey {
    SPACE = 32,
    APOSTROPHE = 39, /* ' */
    COMMA = 44, /* , */
    MINUS = 45, /* - */
    PERIOD = 46, /* . */
    SLASH = 47, /* / */
    NUM_0 = 48,
    NUM_1 = 49,
    NUM_2 = 50,
    NUM_3 = 51,
    NUM_4 = 52,
    NUM_5 = 53,
    NUM_6 = 54,
    NUM_7 = 55,
    NUM_8 = 56,
    NUM_9 = 57,
    SEMICOLON = 59, /* ; */
    EQUAL = 61, /* = */
    A = 65,
    B = 66,
    C = 67,
    D = 68,
    E = 69,
    F = 70,
    G = 71,
    H = 72,
    I = 73,
    J = 74,
    K = 75,
    L = 76,
    M = 77,
    N = 78,
    O = 79,
    P = 80,
    Q = 81,
    R = 82,
    S = 83,
    T = 84,
    U = 85,
    V = 86,
    W = 87,
    X = 88,
    Y = 89,
    Z = 90,
    LEFT_BRACKET = 91, /* [ */
    BACKSLASH = 92, /* \ */
    RIGHT_BRACKET = 93, /* ] */
    GRAVE_ACCENT = 96, /* ` */
    WORLD_1 = 161, /* non-US #1 */
    WORLD_2 = 162, /* non-US #2 */
    ESCAPE = 256,
    ENTER = 257,
    TAB = 258,
    BACKSPACE = 259,
    INSERT = 260,
    DELETE = 261,
    RIGHT = 262,
    LEFT = 263,
    DOWN = 264,
    UP = 265,
    PAGE_UP = 266,
    PAGE_DOWN = 267,
    HOME = 268,
    END = 269,
    CAPS_LOCK = 280,
    SCROLL_LOCK = 281,
    NUM_LOCK = 282,
    PRINT_SCREEN = 283,
    PAUSE = 284,
    F1 = 290,
    F2 = 291,
    F3 = 292,
    F4 = 293,
    F5 = 294,
    F6 = 295,
    F7 = 296,
    F8 = 297,
    F9 = 298,
    F10 = 299,
    F11 = 300,
    F12 = 301,
    F13 = 302,
    F14 = 303,
    F15 = 304,
    F16 = 305,
    F17 = 306,
    F18 = 307,
    F19 = 308,
    F20 = 309,
    F21 = 310,
    F22 = 311,
    F23 = 312,
    F24 = 313,
    F25 = 314,
    KP_0 = 320,
    KP_1 = 321,
    KP_2 = 322,
    KP_3 = 323,
    KP_4 = 324,
    KP_5 = 325,
    KP_6 = 326,
    KP_7 = 327,
    KP_8 = 328,
    KP_9 = 329,
    KP_DECIMAL = 330,
    KP_DIVIDE = 331,
    KP_MULTIPLY = 332,
    KP_SUBTRACT = 333,
    KP_ADD = 334,
    KP_ENTER = 335,
    KP_EQUAL = 336,
    LEFT_SHIFT = 340,
    LEFT_CONTROL = 341,
    LEFT_ALT = 342,
    LEFT_SUPER = 343,
    RIGHT_SHIFT = 344,
    RIGHT_CONTROL = 345,
    RIGHT_ALT = 346,
    RIGHT_SUPER = 347,
    MENU = 348,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GLFWmousebutton {
    BUTTON_1 = 0,
    BUTTON_2 = 1,
    BUTTON_3 = 2,
    BUTTON_4 = 3,
    BUTTON_5 = 4,
    BUTTON_6 = 5,
    BUTTON_7 = 6,
    BUTTON_8 = 7,
}

pub type GLFWerrorfun = extern "C" fn(error_code: c_int, desc: *const c_char);
pub type GLFWkeyfun = extern "C" fn(window: *mut GLFWwindow, key: GLFWkey, scancode: c_int, action: GLFWaction, mods: c_int);
pub type GLFWwindowclosefun = extern "C" fn(window: *mut GLFWwindow);
pub type GLFWcursorposfun = extern "C" fn(window: *mut GLFWwindow, xpos: c_double, ypos: c_double);
pub type GLFWscrollfun = extern "C" fn(window: *mut GLFWwindow, xoffset: c_double, yoffset: c_double);
pub type GLFWmousebuttonfun = extern "C" fn(window: *mut GLFWwindow, button: GLFWmousebutton, action: GLFWaction, mods: c_int);
pub type GLFWframebuffersizefun = extern "C" fn(window: *mut GLFWwindow, width: c_int, height: c_int);

#[link(name = "glfw3")]
extern "C" {
    pub fn glfwInit() -> GLFWbool;
    pub fn glfwTerminate();
    pub fn glfwGetRequiredInstanceExtensions(count: *mut u32) -> *const *const c_char;
   
    /// \[main thread only]
    pub fn glfwCreateWindow(width: c_int, height: c_int, title: *const c_char, monitor: *mut GLFWmonitor, share: *mut GLFWwindow) -> *mut GLFWwindow;
    pub fn glfwWindowHint(hint: c_int, value: c_int);
    /// \[main thread only]
    pub fn glfwDestroyWindow(window: *mut GLFWwindow);
    pub fn glfwGetFramebufferSize(window: *mut GLFWwindow, width: *mut c_int, height: *mut c_int);
    pub fn glfwWindowShouldClose(window: *mut GLFWwindow) -> GLFWbool;
    pub fn glfwSetErrorCallback(callback: GLFWerrorfun) -> GLFWerrorfun;
    pub fn glfwSetKeyCallback(window: *mut GLFWwindow, callback: GLFWkeyfun) -> GLFWkeyfun;
    pub fn glfwSetWindowCloseCallback(window: *mut GLFWwindow, callback: GLFWwindowclosefun) -> GLFWwindowclosefun;
    
    /// \[main thread only]
    pub fn glfwPollEvents();
    pub fn glfwSetWindowShouldClose(window: *mut GLFWwindow, value: GLFWbool);
    pub fn glfwSetCursorPosCallback(window: *mut GLFWwindow, callback: GLFWcursorposfun) -> GLFWcursorposfun;
    pub fn glfwSetScrollCallback(window: *mut GLFWwindow, callback: GLFWscrollfun) -> GLFWscrollfun;
    pub fn glfwSetMouseButtonCallback(window: *mut GLFWwindow, callback: GLFWmousebuttonfun) -> GLFWmousebuttonfun;
    pub fn glfwSetFramebufferSizeCallback(window: *mut GLFWwindow, callback: GLFWframebuffersizefun) -> GLFWframebuffersizefun;

    /// \[main thread only]
    pub fn glfwSetClipboardString(window: *mut GLFWwindow, string: *const c_char);

    /// \[main thread only]
    pub fn glfwGetClipboardString(window: *mut GLFWwindow) -> *const c_char;

    pub fn glfwMakeContextCurrent(window: *mut GLFWwindow);
    pub fn glfwSwapBuffers(window: *mut GLFWwindow);
    pub fn glfwSwapInterval(interval: c_int);

    /// \[main thread only]
    pub fn glfwSetWindowSize(window: *mut GLFWwindow, width: c_int, height: c_int);

    /// \[main thread only]
    pub fn glfwGetWindowSize(window: *mut GLFWwindow, width: *mut c_int, height: *mut c_int);

    /// \[main thread only]
    pub fn glfwSetWindowTitle(window: *mut GLFWwindow, title: *const c_char);

    /// \[main thread only]
    pub fn glfwIconifyWindow(window: *mut GLFWwindow);

    /// \[main thread only]
    pub fn glfwRestoreWindow(window: *mut GLFWwindow);

    /// \[main thread only]
    pub fn glfwMaximizeWindow(window: *mut GLFWwindow);

    /// \[main thread only]
    pub fn glfwRequestWindowAttention(window: *mut GLFWwindow);

    /// \[main thread only]
    pub fn glfwGetPrimaryMonitor() -> *mut GLFWmonitor;
    
    /// \[main thread only]
    pub fn glfwGetVideoMode(monitor: *mut GLFWmonitor) -> *const GLFWvidmode;

    /// \[main thread only]
    pub fn glfwGetWindowMonitor(window: *mut GLFWwindow) -> *mut GLFWmonitor;

    /// \[main thread only]
    pub fn glfwSetWindowMonitor(window: *mut GLFWwindow, monitor: *mut GLFWmonitor, xpos: c_int, ypos: c_int, width: c_int, height: c_int, refreshRate: c_int);

    pub fn glfwGetProcAddress(procname: *const c_char) -> *const c_void;
}

// TODO: Actually do good links here
#[link(name = "X11")]
extern "C" {}
