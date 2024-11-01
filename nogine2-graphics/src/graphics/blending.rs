use crate::gl_wrapper::{gl_additive_blend, gl_alpha_blend, gl_multiplicative_blend, gl_subtractive_blend};

/// Represents the blending modes available for rendering.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendingMode {
    /// Mixes the source and destination colors depending on alpha.
    AlphaMix,

    /// Adds the new color data (premultiplied by alpha) to the old color data.
    Additive,

    /// Subtracts the new color data (premultiplied by alpha) from the old color data.
    Subtractive,

    /// Multiplies the new color data by the old color data (ignores alpha).
    Multiplicative,
}

impl BlendingMode {
    pub(crate) fn apply(&self) {
        match self {
            BlendingMode::AlphaMix => gl_alpha_blend(),
            BlendingMode::Additive => gl_additive_blend(),
            BlendingMode::Subtractive => gl_subtractive_blend(),
            BlendingMode::Multiplicative => gl_multiplicative_blend(),
        }
    }
}
