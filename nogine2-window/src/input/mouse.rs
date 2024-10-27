use nogine2_core::math::vector2::vec2;

use crate::glfw::GLFWmousebutton;

use super::Action;

/// A snapshot of the mouse state at a certain point.
#[derive(Debug, Clone)]
pub struct Mouse {
    states: u16,
    mouse_pos: vec2,
    prev_mouse_pos: vec2,
    scroll: vec2,
}

impl Mouse {
    pub(super) const fn new() -> Self {
        Self { states: 0, mouse_pos: vec2::ZERO, prev_mouse_pos: vec2::ZERO, scroll: vec2::ZERO }
    }

    /// Returns if `button` is being pressed.
    pub fn button(&self, button: Button) -> bool {
        return self.button_state(button).contains(Action::Press);
    }

    /// Returns if `button` has been pressed this frame.
    pub fn button_pressed(&self, button: Button) -> bool {
        return self.button_state(button) == Action::Press;
    }

    /// Returns if `button` has been released this frame.
    pub fn button_released(&self, button: Button) -> bool {
        return self.button_state(button) == Action::Release;
    }

    /// Returns the state of a `button`.
    pub fn button_state(&self, button: Button) -> Action {
        let lshift = button.to_index();
        return Action::from_bits_retain(((self.states >> lshift) & 0b11) as u8);
    }

    /// Returns the mouse position.
    pub fn pos(&self) -> vec2 {
        self.mouse_pos
    }

    /// Returns the mouse position delta.
    pub fn delta(&self) -> vec2 {
        self.mouse_pos - self.prev_mouse_pos
    }

    /// Returns the mouse scrolling.
    pub fn scroll(&self) -> vec2 {
        self.scroll
    }

    pub(super) fn set_button_state(&mut self, button: Button, now_pressed: bool) {
        let lshift = button.to_index();
        self.states &= !(0b10 << lshift);
        self.states |= (now_pressed as u16) << (lshift + 1);
    }

    pub(super) fn set_scroll(&mut self, scroll: vec2) {
        self.scroll = scroll;
    }

    pub(super) fn set_pos(&mut self, pos: vec2) {
        self.mouse_pos = pos;
    }

    pub(super) fn flush(&mut self) {
        const MASK: u16 = 0xAAAA;
        let x = self.states & MASK;
        self.states = x | (x >> 1);

        self.prev_mouse_pos = self.mouse_pos;
        self.scroll = vec2::ZERO;
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Button {
    Left = 0,
    Right = 1,
    Middle = 2,
    Button4 = 3,
    Button5 = 4,
    Button6 = 5,
    Button7 = 6,
    Button8 = 7,
}

impl From<GLFWmousebutton> for Button {
    fn from(value: GLFWmousebutton) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl Button {
    fn to_index(&self) -> usize {
        *self as i32 as usize * 2
    }
}
