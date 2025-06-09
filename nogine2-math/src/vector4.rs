#![allow(non_camel_case_types)]

use std::simd::{cmp::SimdOrd, f32x4, f64x4, i32x4, num::*, u32x4, StdFloat};

use gamedev_math::{cast_vec4_impl, float_vec4_impl, gen_vec4, scalar_vec4_impl, signed_vec4_impl, unsigned_vec4_impl};

use super::{vector2::{bvec2, dvec2, ivec2, uvec2, vec2}, vector3::{bvec3, dvec3, ivec3, uvec3, vec3}};

gen_vec4!(bvec4, bvec3, bvec2, bool, false);

gen_vec4!(ivec4, ivec3, ivec2, i32, 0);
scalar_vec4_impl!(ivec4, i32, i32x4);
unsigned_vec4_impl!(ivec4, i32, 0, 1);
signed_vec4_impl!(ivec4, i32, 0, 1);

gen_vec4!(uvec4, uvec3, uvec2, u32, 0);
scalar_vec4_impl!(uvec4, u32, u32x4);
unsigned_vec4_impl!(uvec4, u32, 0, 1);

gen_vec4!(vec4, vec3, vec2, f32, 0.0);
scalar_vec4_impl!(vec4, f32, f32x4);
unsigned_vec4_impl!(vec4, f32, 0.0, 1.0);
signed_vec4_impl!(vec4, f32, 0.0, 1.0);
float_vec4_impl!(vec4, f32, f32x4);

gen_vec4!(dvec4, dvec3, dvec2, f64, 0.0);
scalar_vec4_impl!(dvec4, f64, f64x4);
unsigned_vec4_impl!(dvec4, f64, 0.0, 1.0);
signed_vec4_impl!(dvec4, f64, 0.0, 1.0);
float_vec4_impl!(dvec4, f64, f64x4);

cast_vec4_impl!(ivec4, i32, uvec4, vec4, dvec4);
cast_vec4_impl!(uvec4, u32, ivec4, vec4, dvec4);
cast_vec4_impl!(vec4, f32, uvec4, ivec4, dvec4);
cast_vec4_impl!(dvec4, f64, uvec4, vec4, ivec4);
