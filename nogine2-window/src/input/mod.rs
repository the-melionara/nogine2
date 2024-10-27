use std::sync::RwLock;

use bitflags::bitflags;
use keyboard::Keyboard;
use nogine2_core::log_error;

use crate::glfw::{GLFWaction, GLFWkey};

pub mod keyboard;

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
}

impl Input {
    const fn new() -> Self {
        Self { keyboard: Keyboard::new() }
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

    pub(crate) fn submit_key(key: GLFWkey, action: GLFWaction) {
        let Ok(mut input) = INPUT.write() else { log_error!("Couldn't access input singleton!"); return };

        input.keyboard.set_key_state(key.into(), match action {
            GLFWaction::RELEASE => false,
            GLFWaction::PRESS => true,
            GLFWaction::REPEAT => return,
        });
    }

    pub(crate) fn flush() {
        let Ok(mut input) = INPUT.write() else { log_error!("Couldn't access input singleton!"); return };
        input.keyboard.flush();
    }
}
