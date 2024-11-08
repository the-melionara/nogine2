use std::{cell::RefCell, sync::Arc};

use nogine2_core::{log_error, main_thread::test_main_thread, math::{vector2::{ivec2, uvec2, vec2}, vector3::{ivec3, uvec3, vec3}, vector4::{ivec4, uvec4, vec4}}};
use uuid::Uuid;

use crate::gl_wrapper::gl_uniform;

use super::shader::Shader;

/// A data type that the required information to use a custom shader with custom uniforms.
#[derive(Debug)]
pub struct Material {
    uuid: Uuid,
    shader: Arc<Shader>,
    uniforms: RefCell<MaterialUniformHolder>,
}

impl Material {
    pub fn new(shader: Arc<Shader>) -> Arc<Self> {
        Arc::new(Self { uuid: Uuid::new_v4(), shader, uniforms: RefCell::new(MaterialUniformHolder::new()) })
    }

    /// Sets the value of a uniform. Will panic if `name` is not zero terminated.
    pub fn set_uniform(&self, name: &[u8], uniform: Uniform) {
        test_main_thread();
        let mut borrow = self.uniforms.borrow_mut();
        if let Some(loc) = self.shader.uniform_loc(name) {
            borrow.set_uniform(loc, uniform);
        }
    }

    pub(crate) fn use_material(&self) -> bool {
        self.shader.use_shader() && self.uniforms.borrow().enable()
    }

    pub(crate) fn uniform_loc(&self, name: &[u8]) -> Option<i32> {
        self.shader.uniform_loc(name)
    }
}

unsafe impl Sync for Material { }
unsafe impl Send for Material { }


#[derive(Debug)]
struct MaterialUniformHolder {
    uniforms: Vec<(i32, Uniform)>,
}

impl MaterialUniformHolder {
    const fn new() -> Self {
        Self { uniforms: Vec::new() }
    }

    fn set_uniform(&mut self, loc: i32, uniform: Uniform) {
        match self.uniforms.iter_mut().position(|(i, _)| *i == loc) {
            Some(i) => self.uniforms[i].1 = uniform,
            None => self.uniforms.push((loc, uniform))
        }
    }

    fn enable(&self) -> bool {
        for (loc, val) in self.uniforms.iter().copied() {
            match val {
                Uniform::Int(x) => gl_uniform::set_i32(loc, x),
                Uniform::IVec2(x) => gl_uniform::set_ivec2(loc, x),
                Uniform::IVec3(x) => gl_uniform::set_ivec3(loc, x),
                Uniform::IVec4(x) => gl_uniform::set_ivec4(loc, x),
                Uniform::Uint(x) => gl_uniform::set_u32(loc, x),
                Uniform::UVec2(x) => gl_uniform::set_uvec2(loc, x),
                Uniform::UVec3(x) => gl_uniform::set_uvec3(loc, x),
                Uniform::UVec4(x) => gl_uniform::set_uvec4(loc, x),
                Uniform::Float(x) => gl_uniform::set_f32(loc, x),
                Uniform::Vec2(x) => gl_uniform::set_vec2(loc, x),
                Uniform::Vec3(x) => gl_uniform::set_vec3(loc, x),
                Uniform::Vec4(x) => gl_uniform::set_vec4(loc, x),
            }

            if gl_uniform::uniforms_failed() {
                log_error!("Couldn't set uniform in location '{loc}' (value was {val:?})!");
                return false;
            }
        }
        return true;
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid 
    }
}

/// A value that can be sent to the GPU as a uniform.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Uniform {
    Int(i32), IVec2(ivec2), IVec3(ivec3), IVec4(ivec4),
    Uint(u32), UVec2(uvec2), UVec3(uvec3), UVec4(uvec4),
    Float(f32), Vec2(vec2), Vec3(vec3), Vec4(vec4),
}

impl Uniform {
    pub fn typ(&self) -> UniformType {
        match self {
            Uniform::Int(_) => UniformType::Int,
            Uniform::IVec2(_) => UniformType::IVec2,
            Uniform::IVec3(_) => UniformType::IVec3,
            Uniform::IVec4(_) => UniformType::IVec4,
            Uniform::Uint(_) => UniformType::Uint,
            Uniform::UVec2(_) => UniformType::UVec2,
            Uniform::UVec3(_) => UniformType::UVec3,
            Uniform::UVec4(_) => UniformType::UVec4,
            Uniform::Float(_) => UniformType::Float,
            Uniform::Vec2(_) => UniformType::Vec2,
            Uniform::Vec3(_) => UniformType::Vec3,
            Uniform::Vec4(_) => UniformType::Vec4,
        }
    }
}

macro_rules! uniform_from_impl {
    ($ident:ident, $typ:ty) => {
        impl From<$typ> for Uniform {
            fn from(value: $typ) -> Self {
                Self::$ident(value)
            }
        }
    };
}

uniform_from_impl!(Int, i32);
uniform_from_impl!(IVec2, ivec2);
uniform_from_impl!(IVec3, ivec3);
uniform_from_impl!(IVec4, ivec4);
uniform_from_impl!(Uint, u32);
uniform_from_impl!(UVec2, uvec2);
uniform_from_impl!(UVec3, uvec3);
uniform_from_impl!(UVec4, uvec4);
uniform_from_impl!(Float, f32);
uniform_from_impl!(Vec2, vec2);
uniform_from_impl!(Vec3, vec3);
uniform_from_impl!(Vec4, vec4);


/// The type of a uniform value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UniformType {
    Int, IVec2, IVec3, IVec4,
    Uint, UVec2, UVec3, UVec4,
    Float, Vec2, Vec3, Vec4,
}
