use std::sync::RwLock;

use bitflags::bitflags;
use controller::{Controller, ControllerMappings};
use keyboard::Keyboard;
use mouse::Mouse;
use nogine2_core::{assert_expr, log_error, math::vector2::vec2};

use crate::glfw::{self, glfwJoystickPresent, GLFWaction, GLFWbool, GLFWkey, GLFWmousebutton, GLFW_CLIENT_API};

pub mod keyboard;
pub mod mouse;
pub mod controller;

/* All button states follow the following scheme:
 * 0b00: Not pressed
 * 0b11: Held
 * 0b10: Pressed
 * 0b01: Released
 *
 * The first bit represents whether the button is currently being pressed, and the second
 * represents whether the button was being pressed the previous frame.
 */

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Action : u8 {
        const None = 0b00;
        const Hold = 0b11;
        const Press = 0b10;
        const Release = 0b01;
    }
}


static INPUT: RwLock<Input> = RwLock::new(Input::new());

pub struct Input {
    keyboard: Keyboard,
    mouse: Mouse,
    controller: Option<Controller>, // Only one, for now
}

impl Input {
    const fn new() -> Self {
        Self { keyboard: Keyboard::new(), mouse: Mouse::new(), controller: None }
    }

    /// Returns a snapshot of the keyboard.
    pub fn keyboard() -> Keyboard {
        match INPUT.write() {
            Ok(x) => x.keyboard.clone(),
            Err(_) => {
                log_error!("Couldn't access input singleton!");
                Keyboard::new()
            }
        }
    }

    /// Returns a snapshot of the mouse.
    pub fn mouse() -> Mouse {
        match INPUT.write() {
            Ok(x) => x.mouse.clone(),
            Err(_) => {
                log_error!("Couldn't access input singleton!");
                Mouse::new()
            }
        }
    }

    /// Returns a snapshot of a controller.
    pub fn controller(id: u8) -> Option<Controller> {
        assert_expr!(id == 0, "Multiple controllers are not supported yet!");
        match INPUT.write() {
            Ok(x) => x.controller.clone(),
            Err(_) => {
                log_error!("Couldn't access input singleton!");
                None
            }
        }
    }

    pub(crate) fn submit_key(key: GLFWkey, action: GLFWaction) {
        let Ok(mut input) = INPUT.write() else { log_error!("Couldn't access input singleton!"); return };

        input.keyboard.set_key_state(key.into(), match action {
            GLFWaction::RELEASE => false,
            GLFWaction::PRESS => true,
            GLFWaction::REPEAT => return,
        });
    }

    pub(crate) fn submit_mouse_pos(pos: vec2) {
        let Ok(mut input) = INPUT.write() else { log_error!("Couldn't access input singleton!"); return };
        input.mouse.set_pos(pos);
    }

    pub(crate) fn submit_mouse_scroll(scroll: vec2) {
        let Ok(mut input) = INPUT.write() else { log_error!("Couldn't access input singleton!"); return };
        input.mouse.set_scroll(scroll);
    }

    pub(crate) fn submit_mouse_button(button: GLFWmousebutton, action: GLFWaction) {
        let Ok(mut input) = INPUT.write() else { log_error!("Couldn't access input singleton!"); return };

        input.mouse.set_button_state(button.into(), match action {
            GLFWaction::RELEASE => false,
            GLFWaction::PRESS => true,
            GLFWaction::REPEAT => return,
        });
    }

    pub(crate) fn flush() {
        let Ok(mut input) = INPUT.write() else { log_error!("Couldn't access input singleton!"); return };
        input.keyboard.flush();
        input.mouse.flush();

        // Update states of controllers
        if let Some(ctrl) = &mut input.controller {
            ctrl.flush();
            ctrl.update(0);

            if unsafe { glfwJoystickPresent(0) == GLFWbool::FALSE } {
                input.controller = None;
            }
        } else {
            if unsafe { glfwJoystickPresent(0) == GLFWbool::TRUE } {
                if let Some(map) = ControllerMappings::new(0) {
                    input.controller = Some(Controller::new(map));
                }
            }
        }
    }
}
