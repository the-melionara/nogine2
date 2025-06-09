#![allow(non_camel_case_types)]

use std::simd::{cmp::SimdOrd, f32x4, f64x4, i32x4, num::*, u32x4, StdFloat};

use gamedev_math::{cast_vec3_impl, float_vec3_impl, gen_vec3, scalar_vec3_impl, signed_vec3_impl, unsigned_vec3_impl, updim_vec3_impl};

use super::vector2::{bvec2, dvec2, ivec2, uvec2, vec2};

gen_vec3!(bvec3, bvec2, bool, false);

gen_vec3!(ivec3, ivec2, i32, 0);
scalar_vec3_impl!(ivec3, i32, i32x4, 0, i32::MIN, i32::MAX);
unsigned_vec3_impl!(ivec3, i32, 0, 1);
signed_vec3_impl!(ivec3, i32, 0, 1);

gen_vec3!(uvec3, uvec2, u32, 0);
scalar_vec3_impl!(uvec3, u32, u32x4, 0, u32::MIN, u32::MAX);
unsigned_vec3_impl!(uvec3, u32, 0, 1);

gen_vec3!(vec3, vec2, f32, 0.0);
scalar_vec3_impl!(vec3, f32, f32x4, 0.0, f32::NEG_INFINITY, f32::INFINITY);
unsigned_vec3_impl!(vec3, f32, 0.0, 1.0);
signed_vec3_impl!(vec3, f32, 0.0, 1.0);
float_vec3_impl!(vec3, f32, f32x4);

gen_vec3!(dvec3, dvec2, f64, 0.0);
scalar_vec3_impl!(dvec3, f64, f64x4, 0.0, f64::NEG_INFINITY, f64::INFINITY);
unsigned_vec3_impl!(dvec3, f64, 0.0, 1.0);
signed_vec3_impl!(dvec3, f64, 0.0, 1.0);
float_vec3_impl!(dvec3, f64, f64x4);

cast_vec3_impl!(ivec3, i32, uvec3, vec3, dvec3);
cast_vec3_impl!(uvec3, u32, ivec3, vec3, dvec3);
cast_vec3_impl!(vec3, f32, uvec3, ivec3, dvec3);
cast_vec3_impl!(dvec3, f64, uvec3, vec3, ivec3);

updim_vec3_impl!(2, bvec3, bool, bvec2);
updim_vec3_impl!(2, ivec3, i32, ivec2);
updim_vec3_impl!(2, uvec3, u32, uvec2);
updim_vec3_impl!(2, vec3, f32, vec2);
updim_vec3_impl!(2, dvec3, f64, dvec2);
