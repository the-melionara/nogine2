use super::Color;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RGBA32(pub f32, pub f32, pub f32, pub f32);

impl Color for RGBA32 {
    const BLACK:         Self = Self(0.0, 0.0, 0.0, 1.0);
    const DARK_RED:      Self = Self(0.5, 0.0, 0.0, 1.0);
    const RED:           Self = Self(1.0, 0.0, 0.0, 1.0);
    const DARK_GREEN:    Self = Self(0.0, 0.5, 0.0, 1.0);
    const DARK_YELLOW:   Self = Self(0.5, 0.5, 0.0, 1.0);
    const ORANGE:        Self = Self(1.0, 0.5, 0.0, 1.0);
    const GREEN:         Self = Self(0.0, 1.0, 0.0, 1.0);
    const LIME:          Self = Self(0.5, 1.0, 0.0, 1.0);
    const YELLOW:        Self = Self(1.0, 1.0, 0.0, 1.0);
    const DARK_BLUE:     Self = Self(0.0, 0.0, 0.5, 1.0);
    const DARK_MAGENTA:  Self = Self(0.5, 0.0, 0.5, 1.0);
    const ROSE:          Self = Self(1.0, 0.0, 0.5, 1.0);
    const DARK_CYAN:     Self = Self(0.0, 0.5, 0.5, 1.0);
    const GRAY:          Self = Self(0.5, 0.5, 0.5, 1.0);
    const LIGHT_RED:     Self = Self(1.0, 0.5, 0.5, 1.0);
    const SPRING_GREEN:  Self = Self(0.0, 1.0, 0.5, 1.0);
    const LIGHT_GREEN:   Self = Self(0.5, 1.0, 0.5, 1.0);
    const LIGHT_YELLOW:  Self = Self(1.0, 1.0, 0.5, 1.0);
    const BLUE:          Self = Self(0.0, 0.0, 1.0, 1.0);
    const VIOLET:        Self = Self(0.5, 0.0, 1.0, 1.0);
    const MAGENTA:       Self = Self(1.0, 0.0, 1.0, 1.0);
    const AZURE:         Self = Self(0.0, 0.5, 1.0, 1.0);
    const LIGHT_BLUE:    Self = Self(0.5, 0.5, 1.0, 1.0);
    const LIGHT_MAGENTA: Self = Self(1.0, 0.5, 1.0, 1.0);
    const CYAN:          Self = Self(0.0, 1.0, 1.0, 1.0);
    const LIGHT_CYAN:    Self = Self(0.5, 1.0, 1.0, 1.0);
    const WHITE:         Self = Self(1.0, 1.0, 1.0, 1.0);
}

impl RGBA32 {
    pub const CLEAR: Self = Self(0.0, 0.0, 0.0, 0.0);
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RGBA8(pub u8, pub u8, pub u8, pub u8);

impl Color for RGBA8 {
    const BLACK:         Self = Self(000, 000, 000, 255);
    const DARK_RED:      Self = Self(127, 000, 000, 255);
    const RED:           Self = Self(255, 000, 000, 255);
    const DARK_GREEN:    Self = Self(000, 127, 000, 255);
    const DARK_YELLOW:   Self = Self(127, 127, 000, 255);
    const ORANGE:        Self = Self(255, 127, 000, 255);
    const GREEN:         Self = Self(000, 255, 000, 255);
    const LIME:          Self = Self(127, 255, 000, 255);
    const YELLOW:        Self = Self(255, 255, 000, 255);
    const DARK_BLUE:     Self = Self(000, 000, 127, 255);
    const DARK_MAGENTA:  Self = Self(127, 000, 127, 255);
    const ROSE:          Self = Self(255, 000, 127, 255);
    const DARK_CYAN:     Self = Self(000, 127, 127, 255);
    const GRAY:          Self = Self(127, 127, 127, 255);
    const LIGHT_RED:     Self = Self(255, 127, 127, 255);
    const SPRING_GREEN:  Self = Self(000, 255, 127, 255);
    const LIGHT_GREEN:   Self = Self(127, 255, 127, 255);
    const LIGHT_YELLOW:  Self = Self(255, 255, 127, 255);
    const BLUE:          Self = Self(000, 000, 255, 255);
    const VIOLET:        Self = Self(127, 000, 255, 255);
    const MAGENTA:       Self = Self(255, 000, 255, 255);
    const AZURE:         Self = Self(000, 127, 255, 255);
    const LIGHT_BLUE:    Self = Self(127, 127, 255, 255);
    const LIGHT_MAGENTA: Self = Self(255, 127, 255, 255);
    const CYAN:          Self = Self(000, 255, 255, 255);
    const LIGHT_CYAN:    Self = Self(127, 255, 255, 255);
    const WHITE:         Self = Self(255, 255, 255, 255);
}

impl RGBA8 {
    pub const CLEAR: Self = Self(000, 000, 000, 000);
}
