#![allow(non_camel_case_types)]

use std::{ops::Neg, simd::{cmp::SimdOrd, f32x2, f64x2, i32x2, num::*, u32x2, StdFloat}};

use gamedev_math::{cast_vec2_impl, float_vec2_impl, gen_vec2, scalar_vec2_impl, signed_vec2_impl, unsigned_vec2_impl};

gen_vec2!(bvec2, bool, false, true);

gen_vec2!(ivec2, i32, 0, 1);
scalar_vec2_impl!(ivec2, i32, i32x2);
unsigned_vec2_impl!(ivec2, i32, 0, 1);
signed_vec2_impl!(ivec2, i32, 0, 1);

gen_vec2!(uvec2, u32, 0, 1);
scalar_vec2_impl!(uvec2, u32, u32x2);
unsigned_vec2_impl!(uvec2, u32, 0, 1);

gen_vec2!(vec2, f32, 0.0, 1.0);
scalar_vec2_impl!(vec2, f32, f32x2);
unsigned_vec2_impl!(vec2, f32, 0.0, 1.0);
signed_vec2_impl!(vec2, f32, 0.0, 1.0);
float_vec2_impl!(vec2, f32, f32x2);

gen_vec2!(dvec2, f64, 0.0, 1.0);
scalar_vec2_impl!(dvec2, f64, f64x2);
unsigned_vec2_impl!(dvec2, f64, 0.0, 1.0);
signed_vec2_impl!(dvec2, f64, 0.0, 1.0);
float_vec2_impl!(dvec2, f64, f64x2);

cast_vec2_impl!(ivec2, i32, uvec2, vec2, dvec2);
cast_vec2_impl!(uvec2, u32, ivec2, vec2, dvec2);
cast_vec2_impl!(vec2, f32, uvec2, ivec2, dvec2);
cast_vec2_impl!(dvec2, f64, uvec2, vec2, ivec2);
