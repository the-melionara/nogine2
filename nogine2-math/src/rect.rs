use gamedev_math::{float_rect_impl, gen_rect};

use super::vector2::{dvec2, ivec2, uvec2, vec2};

gen_rect!(IRect, ivec2, i32, 2);

gen_rect!(URect, uvec2, u32, 2);

gen_rect!(Rect, vec2, f32, 2.0);
float_rect_impl!(Rect, vec2);

gen_rect!(DRect, dvec2, f64, 2.0);
float_rect_impl!(DRect, dvec2);