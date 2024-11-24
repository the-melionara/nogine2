use std::{f32, sync::Arc};

use nogine2_core::math::{vector2::{ivec2, vec2}, vector3::ivec3};

use crate::glfw::{glfwGetJoystickAxes, glfwGetJoystickButtons, glfwGetJoystickGUID, glfwGetJoystickHats, GLFWhat};

use super::Action;

include!(concat!(env!("OUT_DIR"), "/controller_db.rs"));


/// A snapshot of a controller's state at a certain point.
#[derive(Clone)]
pub struct Controller {
    mapping: Arc<ControllerMappings>,
    
    states: u32,
    left_stick: vec2,
    right_stick: vec2,
}

impl Controller {
    pub(super) fn new(mapping: Arc<ControllerMappings>) -> Self {
        Self { mapping, states: 0, left_stick: vec2::ZERO, right_stick: vec2::ZERO }
    }

    pub fn mappings(&self) -> &ControllerMappings {
        &self.mapping
    }

    /// Returns if `button` is being pressed.
    pub fn button(&self, button: ControllerButton) -> bool {
        return self.button_state(button).contains(Action::Press);
    }

    /// Returns if `button` has been pressed this frame.
    pub fn button_pressed(&self, button: ControllerButton) -> bool {
        return self.button_state(button) == Action::Press;
    }

    /// Returns if `button` has been released this frame.
    pub fn button_released(&self, button: ControllerButton) -> bool {
        return self.button_state(button) == Action::Release;
    }

    /// Returns the state of a `button`.
    pub fn button_state(&self, button: ControllerButton) -> Action {
        let layout = self.mapping.layout;
        return self.raw_button_state(button.to_raw(layout));
    }
    
    /// Returns the state of a `button`.
    pub fn raw_button_state(&self, button: RawControllerButton) -> Action {
        let lshift = button.to_index();
        return Action::from_bits_retain(((self.states >> lshift) & 0b11) as u8);
    }
    
    /// Returns an axis from two buttons.
    pub fn axis1(&self, neg: ControllerButton, pos: ControllerButton) -> i32 {
        return self.button(pos) as i32 - self.button(neg) as i32;
    }
    
    /// Returns two axes from four buttons.
    pub fn axis2(&self, neg: (ControllerButton, ControllerButton), pos: (ControllerButton, ControllerButton)) -> ivec2 {
        return ivec2(
            self.button(pos.0) as i32 - self.button(neg.0) as i32,
            self.button(pos.1) as i32 - self.button(neg.1) as i32,
        );
    }

    /// Returns three axes from six buttons.
    pub fn axis3(&self, neg: (ControllerButton, ControllerButton, ControllerButton), pos: (ControllerButton, ControllerButton, ControllerButton)) -> ivec3 {
        return ivec3(
            self.button(pos.0) as i32 - self.button(neg.0) as i32,
            self.button(pos.1) as i32 - self.button(neg.1) as i32,
            self.button(pos.2) as i32 - self.button(neg.2) as i32,
        );
    }

    /// Returns the displacement of the left stick.
    pub fn left_stick(&self) -> vec2 {
        self.left_stick
    }

    /// Returns the displacement of the right stick.
    pub fn right_stick(&self) -> vec2 {
        self.right_stick
    }

    /// Returns the value of the dpad (ensuring a joined dpad where it's impossible to press opposite directions at the same time).
    pub fn dpad(&self) -> ivec2 {
        self.axis2(
            (ControllerButton::DpadLeft, ControllerButton::DpadDown),
            (ControllerButton::DpadRight, ControllerButton::DpadUp),
        )
    }

    pub(super) fn update(&mut self, jid: u32) {
        self.left_stick = vec2(
            self.mapping.left_x.check_as_axis(jid),
            self.mapping.left_y.check_as_axis(jid),
        );
        self.right_stick = vec2(
            self.mapping.right_x.check_as_axis(jid),
            self.mapping.right_y.check_as_axis(jid),
        );
        
        self.set_button_state(RawControllerButton::ACross, self.mapping.a.check_as_button(jid));
        self.set_button_state(RawControllerButton::BCircle, self.mapping.b.check_as_button(jid));
        self.set_button_state(RawControllerButton::XSquare, self.mapping.x.check_as_button(jid));
        self.set_button_state(RawControllerButton::YTriangle, self.mapping.y.check_as_button(jid));

        self.set_button_state(RawControllerButton::R1, self.mapping.r1.check_as_button(jid));
        self.set_button_state(RawControllerButton::R2, self.mapping.r2.check_as_button(jid));
        self.set_button_state(RawControllerButton::R3, self.mapping.r3.check_as_button(jid));
    
        self.set_button_state(RawControllerButton::L1, self.mapping.l1.check_as_button(jid));
        self.set_button_state(RawControllerButton::L2, self.mapping.l2.check_as_button(jid));
        self.set_button_state(RawControllerButton::L3, self.mapping.l3.check_as_button(jid));

        self.set_button_state(RawControllerButton::Start, self.mapping.start.check_as_button(jid));
        self.set_button_state(RawControllerButton::Select, self.mapping.select.check_as_button(jid));

        self.set_button_state(RawControllerButton::DpadLeft, self.mapping.dpad_l.check_as_button(jid));
        self.set_button_state(RawControllerButton::DpadRight, self.mapping.dpad_r.check_as_button(jid));
        self.set_button_state(RawControllerButton::DpadUp, self.mapping.dpad_u.check_as_button(jid));
        self.set_button_state(RawControllerButton::DpadDown, self.mapping.dpad_d.check_as_button(jid));
    }

    fn set_button_state(&mut self, button: RawControllerButton, now_pressed: bool) {
        let lshift = button.to_index();
        self.states &= !(0b10 << lshift);
        self.states |= (now_pressed as u32) << (lshift + 1);
    }

    pub(super) fn flush(&mut self) {
        const MASK: u32 = 0xAAAAAAAA;
        let x = self.states & MASK;
        self.states = x | (x >> 1);
    }
}


struct CtrlDbEntry {
    name: &'static str,
    ctrls: &'static [(&'static str, CtrlDbBinding)]
}

#[derive(Debug, Clone, Copy)]
enum CtrlDbBinding { Missing, Button(u8), Hat(u8, u8), Axis(u8, f32) }

impl CtrlDbBinding {
    fn check_as_button(&self, jid: u32) -> bool {
        match self {
            CtrlDbBinding::Button(button) => unsafe {
                let mut count = 0;
                let ptr = glfwGetJoystickButtons(jid as i32, &mut count);
                if ptr.is_null() || *button as i32 >= count {
                    return false;
                }
                return ptr.add(*button as usize).read() != 0;
            },
            CtrlDbBinding::Hat(0, hat) => unsafe {
                let mut count = 0;
                let ptr = glfwGetJoystickHats(jid as i32, &mut count);
                if ptr.is_null() || count == 0 {
                    return false;
                }
                return !(ptr.read() & GLFWhat::from_bits_retain(*hat)).is_empty();
            },
            CtrlDbBinding::Axis(axis, _) => unsafe {
                let mut count = 0;
                let ptr = glfwGetJoystickAxes(jid as i32, &mut count);
                if ptr.is_null() || *axis as i32 >= count {
                    return false;
                }
                return ptr.add(*axis as usize).read().abs() > f32::EPSILON;
            },
            _ => false,
        }
    }

    fn check_as_axis(&self, jid: u32) -> f32 {
        match self {
            CtrlDbBinding::Hat(0, hat) => unsafe {
                let mut count = 0;
                let ptr = glfwGetJoystickHats(jid as i32, &mut count);
                if ptr.is_null() || count != 0 {
                    return 0.0;
                }

                let hat_val = ptr.read() & GLFWhat::from_bits_retain(*hat);
                return if hat_val.intersects(GLFWhat::LEFT | GLFWhat::UP) {
                    -1.0
                } else if hat_val.intersects(GLFWhat::RIGHT | GLFWhat::DOWN) {
                    1.0
                } else {
                    0.0
                };
            },
            CtrlDbBinding::Axis(axis, mult) => unsafe {
                let mut count = 0;
                let ptr = glfwGetJoystickAxes(jid as i32, &mut count);
                if ptr.is_null() || *axis as i32 >= count {
                    return 0.0;
                }
                return ptr.add(*axis as usize).read() * mult;
            },
            _ => 0.0,
        }
    }
}


/// Defines the layout of a controller.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControllerLayout {
    Playstation,
    Nintendo,
    Xbox
}


/// Defines the model of a controller.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControllerModel {
    PS2, PS3, PS4, PS5,
    WiiClassic, WiiUPro, SwitchPro, JoyConLeft, JoyConRight, JoyConPair,
    Xbox360, XboxOne, XboxSeriesX,
    SteamDeck,
    Unknown(ControllerLayout),
}

impl ControllerModel {
    pub fn new(name: &str) -> Self {
        if name.contains("PS2") || name.contains("DualShock 2") {
            return Self::PS2;
        }
        if name.contains("PS3") || name.contains("DualShock 3") {
            return Self::PS3;
        }
        if name.contains("PS4") || name.contains("DualShock 4") {
            return Self::PS4;
        }
        if name.contains("PS5") || name.contains("DualShock 5") {
            return Self::PS5;
        }
        if name.contains("Playstation") {
            return Self::Unknown(ControllerLayout::Playstation);
        }

        if name.contains("Xbox Series") {
            return Self::XboxSeriesX;
        }
        if name.contains("Xbox One") {
            return Self::XboxOne;
        }
        if name.contains("360") {
            return Self::Xbox360;
        }

        if name.contains("Joy-Con") {
            if name.contains("Left") || name.contains("(L)") {
                return Self::JoyConLeft;
            }
            if name.contains("Right") || name.contains("(R)") {
                return Self::JoyConRight;
            }
            if name.contains("Combined") || name.contains("(L/R)") {
                return Self::JoyConPair;
            }
            return Self::Unknown(ControllerLayout::Nintendo);
        }
        if name.contains("Switch") {
            return Self::SwitchPro;
        }
        if name.contains("Wii U") {
            return Self::WiiUPro;
        }
        if name.contains("Wii Classic") {
            return Self::WiiClassic;
        }
        if name.contains("Nintendo") || name.contains("Wii") {
            return Self::Unknown(ControllerLayout::Nintendo);
        }

        return Self::Unknown(ControllerLayout::Xbox);
    }
    
    pub fn layout(&self) -> ControllerLayout {
        match self {
            ControllerModel::PS2 |
            ControllerModel::PS3 |
            ControllerModel::PS4 |
            ControllerModel::PS5 => ControllerLayout::Playstation,
            ControllerModel::WiiClassic |
            ControllerModel::WiiUPro |
            ControllerModel::SwitchPro |
            ControllerModel::JoyConLeft |
            ControllerModel::JoyConRight |
            ControllerModel::JoyConPair => ControllerLayout::Nintendo,
            ControllerModel::Unknown(x) => *x,
            _ => ControllerLayout::Xbox,
        }
    }
}


/// Holds all the mappings of a controller.
#[derive(Debug)]
pub struct ControllerMappings {
    model: ControllerModel,
    layout: ControllerLayout,
    name: String,
    
    a: CtrlDbBinding,
    b: CtrlDbBinding,
    x: CtrlDbBinding,
    y: CtrlDbBinding,

    r1: CtrlDbBinding,
    r2: CtrlDbBinding,
    r3: CtrlDbBinding,
    
    l1: CtrlDbBinding,
    l2: CtrlDbBinding,
    l3: CtrlDbBinding,

    start: CtrlDbBinding,
    select: CtrlDbBinding,

    dpad_l: CtrlDbBinding,
    dpad_r: CtrlDbBinding,
    dpad_u: CtrlDbBinding,
    dpad_d: CtrlDbBinding,

    left_x: CtrlDbBinding,
    left_y: CtrlDbBinding,
    right_x: CtrlDbBinding,
    right_y: CtrlDbBinding,
}

impl ControllerMappings {
    pub fn new(jid: u32) -> Option<Arc<Self>> {
        let mut res = Self::default();
        let guid = unsafe { std::ffi::CStr::from_ptr(glfwGetJoystickGUID(jid as i32)) }.to_str().unwrap();
        if let Some((_, entry)) = CONTROLLER_DB.iter().find(|(name, _)| *name == guid) {
            res.model = ControllerModel::new(entry.name);
            res.layout = res.model.layout();
            res.name = entry.name.to_string();

            for (ctrl, bind) in entry.ctrls {
                match *ctrl {
                    "a" => res.a = *bind,
                    "b" => res.b = *bind,
                    "x" => res.x = *bind,
                    "y" => res.y = *bind,
                    "dpdown" => res.dpad_d = *bind,
                    "dpup" => res.dpad_u = *bind,
                    "dpright" => res.dpad_r = *bind,
                    "dpleft" => res.dpad_l = *bind,
                    "leftshoulder" => res.l1 = *bind,
                    "lefttrigger" => res.l2 = *bind,
                    "leftstick" => res.l3 = *bind,
                    "rightshoulder" => res.r1 = *bind,
                    "righttrigger" => res.r2 = *bind,
                    "rightstick" => res.r3 = *bind,
                    "leftx" => res.left_x = *bind,
                    "lefty" => res.left_y = *bind,
                    "rightx" => res.right_x = *bind,
                    "righty" => res.right_y = *bind,
                    "start" => res.start = *bind,
                    "back" => res.select = *bind,
                    _ => {},
                }
            }

            if res.layout == ControllerLayout::Nintendo {
                std::mem::swap(&mut res.a, &mut res.b);
                std::mem::swap(&mut res.x, &mut res.y);
            }

            return Some(Arc::new(res));
        }

        return None;
    }
    
    pub fn model(&self) -> ControllerModel {
        self.model
    }

    pub fn layout(&self) -> ControllerLayout {
        self.layout
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Default for ControllerMappings {
    fn default() -> Self {
        Self {
            model: ControllerModel::Unknown(ControllerLayout::Xbox),
            layout: ControllerLayout::Xbox,
            name: String::new(),
    
            a: CtrlDbBinding::Missing,
            b: CtrlDbBinding::Missing,
            x: CtrlDbBinding::Missing,
            y: CtrlDbBinding::Missing,

            r1: CtrlDbBinding::Missing,
            r2: CtrlDbBinding::Missing,
            r3: CtrlDbBinding::Missing,
    
            l1: CtrlDbBinding::Missing,
            l2: CtrlDbBinding::Missing,
            l3: CtrlDbBinding::Missing,

            start: CtrlDbBinding::Missing,
            select: CtrlDbBinding::Missing,

            dpad_l: CtrlDbBinding::Missing,
            dpad_r: CtrlDbBinding::Missing,
            dpad_u: CtrlDbBinding::Missing,
            dpad_d: CtrlDbBinding::Missing,

            left_x: CtrlDbBinding::Missing,
            left_y: CtrlDbBinding::Missing,
            right_x: CtrlDbBinding::Missing,
            right_y: CtrlDbBinding::Missing,
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub enum ControllerButton {
    A, B, X, Y,
    L1, L2, L3,
    R1, R2, R3,
    Start, Select,
    DpadLeft, DpadRight, DpadUp, DpadDown,
    North, South, East, West,
}

impl ControllerButton {
    /// Alias of A for playstation controllers.
    pub const CROSS: Self = Self::A;
    /// Alias of B for playstation controllers.
    pub const CIRCLE: Self = Self::B;
    /// Alias of X for playstation controllers.
    pub const SQUARE: Self = Self::X;
    /// Alias of Y for playstation controllers.
    pub const TRIANGLE: Self = Self::Y;
    
    pub fn to_raw(&self, layout: ControllerLayout) -> RawControllerButton {
        match (self, layout) {
            (Self::L1, _) => RawControllerButton::L1,
            (Self::L2, _) => RawControllerButton::L2,
            (Self::L3, _) => RawControllerButton::L3,
            (Self::R1, _) => RawControllerButton::R1,
            (Self::R2, _) => RawControllerButton::R2,
            (Self::R3, _) => RawControllerButton::R3,
            (Self::Start, _) => RawControllerButton::Start,
            (Self::Select, _) => RawControllerButton::Select,
            (Self::DpadLeft, _) => RawControllerButton::DpadLeft,
            (Self::DpadRight, _) => RawControllerButton::DpadRight,
            (Self::DpadUp, _) => RawControllerButton::DpadUp,
            (Self::DpadDown, _) => RawControllerButton::DpadDown,
            (Self::A, _) => RawControllerButton::ACross,
            (Self::B, _) => RawControllerButton::BCircle,
            (Self::X, _) => RawControllerButton::XSquare,
            (Self::Y, _) => RawControllerButton::YTriangle,
            (Self::North, ControllerLayout::Nintendo) => RawControllerButton::XSquare,
            (Self::South, ControllerLayout::Nintendo) => RawControllerButton::BCircle,
            (Self::East, ControllerLayout::Nintendo) => RawControllerButton::ACross,
            (Self::West, ControllerLayout::Nintendo) => RawControllerButton::YTriangle,
            (Self::North, _) => RawControllerButton::YTriangle,
            (Self::South, _) => RawControllerButton::ACross,
            (Self::East, _) => RawControllerButton::BCircle,
            (Self::West, _) => RawControllerButton::XSquare,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RawControllerButton {
    ACross, BCircle, XSquare, YTriangle,
    L1, L2, L3,
    R1, R2, R3,
    Start, Select,
    DpadLeft, DpadRight, DpadUp, DpadDown,
}

impl RawControllerButton {
    fn to_index(&self) -> usize {
        *self as usize * 2
    }
}
