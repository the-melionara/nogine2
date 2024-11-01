use crate::gl_wrapper::{gl_additive_blend, gl_alpha_blend};

/// Represents the blending modes available for rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendingMode {
    AlphaMix,
    Additive,
}

impl BlendingMode {
    pub(crate) fn apply(&self) {
        match self {
            BlendingMode::AlphaMix => gl_alpha_blend(),
            BlendingMode::Additive => gl_additive_blend(),
        }
    }
}
