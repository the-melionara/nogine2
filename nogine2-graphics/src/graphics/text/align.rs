pub enum HorTextAlign {
    Left, Center, Right
}

impl HorTextAlign {
    pub fn initial_dx(&self, extents_width: f32, min_width: f32) -> f32 {
        match self {
            HorTextAlign::Left => 0.0,
            HorTextAlign::Center => (extents_width - min_width) * 0.5,
            HorTextAlign::Right => extents_width - min_width,
        }
    }
}
