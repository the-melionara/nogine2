use std::{ffi::c_void, sync::Arc};

use nogine2_core::math::vector2::uvec2;
use pixels::{PixelFormat, Pixels};

use crate::gl_wrapper::texture::{GlTexture, GlTextureFiltering, GlTextureWrapping};

pub mod pixels;
pub mod rendertex;
pub mod sprite;

/// The handle of a texture. **Must only be used on the main thread!**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextureHandle {
    gl_obj: Arc<GlTexture>,
}

impl TextureHandle {
    pub(crate) fn bind_to(&self, target: u32) {
        self.gl_obj.bind_to(target);
    }

    pub(crate) fn dims(&self) -> uvec2 {
        self.gl_obj.dims()
    }
}


/// A 2D texture. **Must only be used on the main thread!**
#[derive(Debug, Clone)]
pub struct Texture2D {
    gl_obj: Arc<GlTexture>,
    sampling: TextureSampling,
    pixels: Option<Pixels>,
    dims: uvec2,
    format: PixelFormat,
}

impl Texture2D {
    /// Loads a texture from a reader.
    #[cfg(feature = "image-loading")]
    pub fn load(reader: impl AsRef<std::path::Path>, sampling: TextureSampling) -> Result<Self, pixels::PixelLoadingError> {
        let pixel_data = Pixels::load(reader)?;
        return Ok(Self::new(pixel_data, sampling));
    }

    /// Creates a new texture.
    pub fn new(pixel_data: Pixels, sampling: TextureSampling) -> Self {
        let gl_obj = Arc::new(GlTexture::new(
            pixel_data.format().into(), pixel_data.dims(),
            sampling.filtering.into(), sampling.wrapping.into(),
            pixel_data.data().as_ptr() as *const c_void
        ));

        return Self { gl_obj, sampling, dims: pixel_data.dims(), format: pixel_data.format(), pixels: Some(pixel_data) };
    }

    /// Returns a reference to the pixel data from the pixel. It may not be available if the texture has been modified from the GPU.
    pub fn pixels(&self) -> Option<&Pixels> {
        self.pixels.as_ref()
    }

    /// Returns a mutable reference to the pixel data from the pixel. It may not be available if the texture has been modified from the GPU.
    pub fn pixels_mut(&mut self) -> Option<&mut Pixels> {
        self.pixels.as_mut()
    }

    /// Returns a handle to the texture.
    pub fn handle(&self) -> TextureHandle {
        TextureHandle { gl_obj: self.gl_obj.clone() }
    }

    /// Updates the data from the GPU to match the data from the CPU.
    pub fn refresh(&self) {
        if let Some(pixels) = &self.pixels {
            self.gl_obj.set(uvec2::ZERO, pixels.dims(), pixels.format().into(), pixels.data().as_ptr() as *const c_void);
        }
    }

    /// Returns the resolution of the texture.
    pub fn dims(&self) -> uvec2 {
        self.dims
    }

    pub fn sampling(&self) -> &TextureSampling {
        &self.sampling
    }

    pub fn pixel_format(&self) -> PixelFormat{
        self.format
    }
}

impl PartialEq for Texture2D {
    fn eq(&self, other: &Self) -> bool {
        self.gl_obj == other.gl_obj
    }
}

impl Eq for Texture2D {}


/// Defines how a texture is sampled.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextureSampling {
    pub filtering: TextureFiltering,
    pub wrapping: TextureWrapping,
}


/// Defines the filtering mode used to sample a texture.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureFiltering {
    /// Does not interpolate between the pixels.
    Nearest = GlTextureFiltering::GlNearest as u32,

    /// Linearly interpolates between pixels.
    Linear = GlTextureFiltering::GlLinear as u32,
}

impl From<TextureFiltering> for GlTextureFiltering {
    fn from(value: TextureFiltering) -> Self {
        unsafe { std::mem::transmute(value) } 
    }
}


/// Defines the wrapping mode used to sample a texture.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureWrapping {
    /// Clamps to the nearest border if it's sampled outside the 0-1 UV range.
    Clamp = GlTextureWrapping::GlClamp as u32,

    /// Repeats the texture if it's sampled outside the 0-1 UV range.
    Repeat = GlTextureWrapping::GlRepeat as u32,

    /// Repeats the texture mirrored if it's sampled outside the 0-1 UV range.
    MirroredRepeat = GlTextureWrapping::GlMirroredRepeat as u32,
}

impl From<TextureWrapping> for GlTextureWrapping {
    fn from(value: TextureWrapping) -> Self {
        unsafe { std::mem::transmute(value) } 
    }
}
