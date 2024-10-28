pub mod rgba;

pub trait Color {
    /// #000000
    const BLACK: Self;
    /// #7F0000
    const DARK_RED: Self;
    /// #FF0000
    const RED: Self;
    /// #007F00
    const DARK_GREEN: Self;
    /// #7F7F00
    const DARK_YELLOW: Self;
    /// #FF7F00
    const ORANGE: Self;
    /// #00FF00
    const GREEN: Self;
    /// #7FFF00
    const LIME: Self;
    /// #FFFF00
    const YELLOW: Self;
    /// #00007F
    const DARK_BLUE: Self;
    /// #7F007F
    const DARK_MAGENTA: Self;
    /// #FF007F
    const ROSE: Self;
    /// #007F7F
    const DARK_CYAN: Self;
    /// #7F7F7F
    const GRAY: Self;
    /// #FF7F7F
    const LIGHT_RED: Self;
    /// #00FF7F
    const SPRING_GREEN: Self;
    /// #7FFF7F
    const LIGHT_GREEN: Self;
    /// #FFFF7F
    const LIGHT_YELLOW: Self;
    /// #0000FF
    const BLUE: Self;
    /// #7F00FF
    const VIOLET: Self;
    /// #FF00FF
    const MAGENTA: Self;
    /// #007FFF
    const AZURE: Self;
    /// #7F7FFF
    const LIGHT_BLUE: Self;
    /// #FF7FFF
    const LIGHT_MAGENTA: Self;
    /// #00FFFF
    const CYAN: Self;
    /// #7FFFFF
    const LIGHT_CYAN: Self;
    /// #FFFFFF
    const WHITE: Self;
}
