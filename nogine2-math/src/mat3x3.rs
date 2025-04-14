#![allow(non_camel_case_types)]

use gamedev_math::{gen_mat3x3, impl_tf3x3};

use super::{vector2::vec2, vector3::vec3};

gen_mat3x3!(mat3, vec3, f32);
impl_tf3x3!(mat3, vec2, f32);
