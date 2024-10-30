use nogine2_core::math::{vector2::ivec2, vector3::ivec3};

use crate::glfw::GLFWkey;

use super::Action;

/// A snapshot of the keyboard state at a certain point.
#[derive(Debug, Clone)]
pub struct Keyboard {
    states: [u64; 4],
}

impl Keyboard {
    pub(super) const fn new() -> Self {
        Self { states: [0; 4] }
    }

    /// Returns if `key` is being pressed.
    pub fn key(&self, key: Key) -> bool {
        return self.key_state(key).contains(Action::Press);
    }

    /// Returns if `key` has been pressed this frame.
    pub fn key_pressed(&self, key: Key) -> bool {
        return self.key_state(key) == Action::Press;
    }

    /// Returns if `key` has been released this frame.
    pub fn key_released(&self, key: Key) -> bool {
        return self.key_state(key) == Action::Release;
    }

    /// Returns the state of a `key`.
    pub fn key_state(&self, key: Key) -> Action {
        let (index, lshift) = key.to_indices();
        return Action::from_bits_retain(((self.states[index] >> lshift) & 0b11) as u8);
    }

    /// Returns an axis from two keys.
    pub fn axis1(&self, neg: Key, pos: Key) -> i32 {
        return self.key(pos) as i32 - self.key(neg) as i32;
    }
    
    /// Returns two axes from four keys.
    pub fn axis2(&self, neg: (Key, Key), pos: (Key, Key)) -> ivec2 {
        return ivec2(
            self.key(pos.0) as i32 - self.key(neg.0) as i32,
            self.key(pos.1) as i32 - self.key(neg.1) as i32,
        );
    }

    /// Returns three axes from six keys.
    pub fn axis3(&self, neg: (Key, Key, Key), pos: (Key, Key, Key)) -> ivec3 {
        return ivec3(
            self.key(pos.0) as i32 - self.key(neg.0) as i32,
            self.key(pos.1) as i32 - self.key(neg.1) as i32,
            self.key(pos.2) as i32 - self.key(neg.2) as i32,
        );
    }

    pub(super) fn set_key_state(&mut self, key: Key, now_pressed: bool) {
        let (index, lshift) = key.to_indices();
        self.states[index] &= !(0b10 << lshift);
        self.states[index] |= (now_pressed as u64) << (lshift + 1);
    }

    pub(super) fn flush(&mut self) {
        const MASK: u64 = 0xAAAAAAAAAAAAAAAA;
        for s in &mut self.states {
            let x = *s & MASK;
            *s = x | (x >> 1);
        }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Unknown = -1,
    Space = 0,
    /// '
    Apostrophe = 1,
    /// ,
    Comma = 2,
    /// -
    Minus = 3,
    /// .
    Period = 4,
    /// /
    Slash = 5,
    Num0 = 6,
    Num1 = 7,
    Num2 = 8,
    Num3 = 9,
    Num4 = 10,
    Num5 = 11,
    Num6 = 12,
    Num7 = 13,
    Num8 = 14,
    Num9 = 15,
    /// ;
    Semicolon = 16,
    /// =
    Equal = 17,
    A = 18,
    B = 19,
    C = 20,
    D = 21,
    E = 22,
    F = 23,
    G = 24,
    H = 25,
    I = 26,
    J = 27,
    K = 28,
    L = 29,
    M = 30,
    N = 31,
    O = 32,
    P = 33,
    Q = 34,
    R = 35,
    S = 36,
    T = 37,
    U = 38,
    V = 39,
    W = 40,
    X = 41,
    Y = 42,
    Z = 43,
    /// [
    LeftBracket = 44,
    /// \
    Backslash = 45,
    /// ]
    RightBracket = 46,
    /// `
    GraveAccent = 47,
    /// Non-US #1
    World1 = 48,
    /// Non-US #2
    World2 = 49,
    Escape = 50,
    Enter = 51,
    Tab = 52,
    Backspace = 53,
    Insert = 54,
    Delete = 55,
    Right = 56,
    Left = 57,
    Down = 58,
    Up = 59,
    PageUp = 60,
    PageDown = 61,
    Home = 62,
    End = 63,
    CapsLock = 64,
    ScrollLock = 65,
    NumLock = 66,
    PrintScreen = 67,
    Pause = 68,
    F1 = 69,
    F2 = 70,
    F3 = 71,
    F4 = 72,
    F5 = 73,
    F6 = 74,
    F7 = 75,
    F8 = 76,
    F9 = 77,
    F10 = 78,
    F11 = 79,
    F12 = 80,
    F13 = 81,
    F14 = 82,
    F15 = 83,
    F16 = 84,
    F17 = 85,
    F18 = 86,
    F19 = 87,
    F20 = 88,
    F21 = 89,
    F22 = 90,
    F23 = 91,
    F24 = 92,
    F25 = 93,
    Keypad0 = 94,
    Keypad1 = 95,
    Keypad2 = 96,
    Keypad3 = 97,
    Keypad4 = 98,
    Keypad5 = 99,
    Keypad6 = 100,
    Keypad7 = 101,
    Keypad8 = 102,
    Keypad9 = 103,
    KeypadDecimal = 104,
    KeypadDivide = 105,
    KeypadMultiply = 106,
    KeypadSubtract = 107,
    KeypadAdd = 108,
    KeypadEnter = 109,
    KeypadEqual = 110,
    LeftShift = 111,
    LeftControl = 112,
    LeftAlt = 113,
    LeftSuper = 114,
    RightShift = 115,
    RightControl = 116,
    RightAlt = 117,
    RightSuper = 118,
    Menu = 119,
}

impl From<GLFWkey> for Key {
    fn from(value: GLFWkey) -> Self {
        let glfw_int_val = value as i32;
        let int_val = match (glfw_int_val, glfw_int_val) {
            (32, _) => 0,
            (39, _) => 1,
            (44..=57, x) => x - 44 + 2,
            (59, _) => 16,
            (61, _) => 17,
            (65..=93, x) => x - 65 + 18,
            (96, _) => 47,
            (161, _) => 48,
            (162, _) => 49,
            (256..=269, x) => x - 256 + 50,
            (280..=284, x) => x - 280 + 64,
            (290..=314, x) => x - 290 + 69,
            (320..=336, x) => x - 320 + 94,
            (340..=348, x) => x - 340 + 111,
            _ => -1,
        };
        return unsafe { std::mem::transmute(int_val) };
    }
}

impl Key {
    /// Index + left shift offset
    fn to_indices(&self) -> (usize, u64) {
        let int_val = *self as i32;
        return ((int_val / 32) as usize, (int_val % 32) as u64 * 2);
    }
}
