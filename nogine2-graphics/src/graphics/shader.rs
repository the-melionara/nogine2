use std::{ffi::CStr, sync::Arc};

use nogine2_core::{assert_expr, main_thread::test_main_thread};

use crate::gl_wrapper::{gl_uniform_loc, program::GlProgram, shader::{GlShader, GlShaderType}};

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubShaderType {
    Vertex = GlShaderType::GlVertexShader as u32,
    Fragment = GlShaderType::GlFragmentShader as u32,
}

impl From<SubShaderType> for GlShaderType {
    fn from(value: SubShaderType) -> Self {
        unsafe { std::mem::transmute(value) }
    } 
}

impl From<GlShaderType> for SubShaderType {
    fn from(value: GlShaderType) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}


/// Represents a single shader stage.
#[derive(Debug, PartialEq, Eq)]
pub struct SubShader {
    gl_obj: GlShader,
}

impl SubShader {
    /// Creates a new subshader. `src` must not be zero-terminated. Will return `None` if compilation failed.
    pub fn new(src: &[u8], typ: SubShaderType) -> Option<Arc<Self>> {
        let gl_obj = GlShader::new(typ.into(), src)?;
        return Some(Arc::new(Self { gl_obj }));
    }

    /// Returns the subshader type.
    pub fn typ(&self) -> SubShaderType {
        self.gl_obj.typ().into()
    }
}


/// Represents a full shader program.
#[derive(Debug, PartialEq, Eq)]
pub struct Shader {
    gl_obj: GlProgram,
    samplers: Vec<u32>,
}

impl Shader {
    /// Creates a new shader. Will panic if `vert` is not a vertex shader or `frag` is not a fragment shader. Will return `None` if linking failed.
    pub fn new(vert: &SubShader, frag: &SubShader) -> Option<Arc<Self>> {
        assert_expr!(vert.typ() == SubShaderType::Vertex, "Vertex subshader must actually be a vertex subshader!");
        assert_expr!(frag.typ() == SubShaderType::Fragment, "Vertex subshader must actually be a vertex subshader!");

        let gl_obj = GlProgram::new(&[&vert.gl_obj, &frag.gl_obj])?;
        let samplers = gl_obj.get_samplers();
        
        return Some(Arc::new(Self { gl_obj, samplers }));
    }

    pub(crate) fn use_shader(&self) -> bool {
        self.gl_obj.use_program()
    }

    pub(crate) fn uniform_loc(&self, name: &CStr) -> Option<i32> {
        test_main_thread();
        return gl_uniform_loc(&self.gl_obj, name);
    }

    pub(crate) fn sampler_index(&self, location: i32) -> Option<usize> {
        return self.samplers.iter().position(|x| *x as i32 == location);
    }

    pub(crate) fn sampler_count(&self) -> usize {
        return self.samplers.len();
    }
}
