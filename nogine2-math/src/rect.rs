use gamedev_math::{float_rect_impl, gen_rect};

use super::vector2::{dvec2, ivec2, uvec2, vec2};

gen_rect!(IRect, ivec2, i32, 2);

gen_rect!(URect, uvec2, u32, 2);

gen_rect!(Rect, vec2, f32, 2.0);
float_rect_impl!(Rect, vec2);

gen_rect!(DRect, dvec2, f64, 2.0);
float_rect_impl!(DRect, dvec2);


impl IRect {
    pub fn from_points(a: ivec2, b: ivec2) -> Self {
        Self { start: a.min(b), end: a.max(b) }
    }
}

impl URect {
    pub fn from_points(a: uvec2, b: uvec2) -> Self {
        Self { start: a.min(b), end: a.max(b) }
    }
}

impl Rect {
    pub fn from_points(a: vec2, b: vec2) -> Self {
        Self { start: a.min(b), end: a.max(b) }
    }
}

impl DRect {
    pub fn from_points(a: dvec2, b: dvec2) -> Self {
        Self { start: a.min(b), end: a.max(b) }
    }
}

impl Rect {
    pub fn intersects(self, other: Self) -> bool {
        return self.start.cle(other.end).all()
            && other.start.cle(self.end).all();
    }
}
