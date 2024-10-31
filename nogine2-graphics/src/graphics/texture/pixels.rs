use nogine2_core::{assert_expr, math::vector2::uvec2};

use crate::{colors::rgba::RGBA8, gl_wrapper::texture::GlTextureFormat};

#[cfg(feature = "image-loading")]
#[derive(Debug)]
pub enum PixelLoadingError {
    IOError(std::io::Error),
    ImageError(image::ImageError),
    UnsupportedFormat
}

#[cfg(feature = "image-loading")]
impl std::fmt::Display for PixelLoadingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PixelLoadingError::IOError(error) => write!(f, "{error}"),
            PixelLoadingError::ImageError(image_error) => write!(f, "{image_error}"),
            PixelLoadingError::UnsupportedFormat => write!(f, "Unsupported Image Format"),
        }
    }
}

#[cfg(feature = "image-loading")]
impl std::error::Error for PixelLoadingError { }


/// Represents the pixel data of an image.
#[derive(Debug, Clone)]
pub struct Pixels {
    data: Vec<u8>,
    dims: uvec2,
    format: PixelFormat,
}

impl Pixels {
    /// Loads the pixel data from a reader.
    #[cfg(feature = "image-loading")]
    pub fn load(path: impl AsRef<std::path::Path>) -> Result<Self, PixelLoadingError> {
        use image::{ColorType, ImageReader};

        let img = ImageReader::open(path).map_err(|e| PixelLoadingError::IOError(e))?.decode().map_err(|e| PixelLoadingError::ImageError(e))?.flipv();
        let dims = uvec2(img.width(), img.height());
        
        match img.color() {
            ColorType::L8 | ColorType::L16 => {
                let data = img.into_luma8();
                let data = data.pixels().flat_map(|x| x.0.iter()).copied().collect::<Vec<_>>();
                return Ok(Self::new(data, dims, PixelFormat::R8));
            },
            ColorType::La8 | ColorType::La16 => {
                let data = img.into_luma_alpha8();
                let data = data.pixels().flat_map(|x| x.0.iter()).copied().collect::<Vec<_>>();
                return Ok(Self::new(data, dims, PixelFormat::RG8));
            },
            ColorType::Rgb8 | ColorType::Rgb16 | ColorType::Rgb32F |
            ColorType::Rgba8 | ColorType::Rgba16 | ColorType::Rgba32F => {
                let data = img.into_rgba8();
                let data = data.pixels().flat_map(|x| x.0.iter()).copied().collect::<Vec<_>>();
                return Ok(Self::new(data, dims, PixelFormat::RGBA8));
            },
            _ => return Err(PixelLoadingError::UnsupportedFormat),
        }
    }

    /// Creates a new `Pixels` struct. Will panic if there's a mismatch between the `data` size and the rest of parameters.
    pub fn new(data: Vec<u8>, dims: uvec2, format: PixelFormat) -> Self {
        assert_expr!(data.len() == (dims.0 * dims.1) as usize * format.byte_size(), "The size of 'data' must be congruent with the 'dims' and 'format' parameters!");
        return Self { data, dims, format };
    }

    pub fn dims(&self) -> uvec2 {
        self.dims
    }

    pub fn format(&self) -> PixelFormat {
        self.format
    }

    /// Returns the color at the selected pixel. Will panic if `pixel` is out of bounds.
    pub fn get_rgba8(&self, pixel: uvec2) -> RGBA8 {
        assert_expr!(pixel.0 < self.dims.0 && pixel.1 < self.dims.1, "Index out of bounds (index was {pixel}, dims were {})!", self.dims);
        
        let offset = (pixel.0 + pixel.1 * self.dims.0) as usize * self.format.byte_size();
        return match self.format {
            PixelFormat::R8 => RGBA8(self.data[offset], 0, 0, 255),
            PixelFormat::RG8 => RGBA8(self.data[offset], self.data[offset + 1], 0, 255),
            PixelFormat::RGBA8 => RGBA8(self.data[offset], self.data[offset + 1], self.data[offset + 2], self.data[offset + 3]),
        }
    }

    /// Sets the color of the selected pixel. Will panic if `pixel` is out of bounds.
    pub fn set_rgba8(&mut self, pixel: uvec2, color: RGBA8) {
        assert_expr!(pixel.0 < self.dims.0 && pixel.1 < self.dims.1, "Index out of bounds (index was {pixel}, dims were {})!", self.dims);
        
        let offset = (pixel.0 + pixel.1 * self.dims.0) as usize * self.format.byte_size();
        return match self.format {
            PixelFormat::R8 => self.data[offset] = color.0,
            PixelFormat::RG8 => { self.data[offset] = color.0; self.data[offset + 1] = color.1 },
            PixelFormat::RGBA8 => {
                self.data[offset] = color.0;
                self.data[offset + 1] = color.1;
                self.data[offset + 2] = color.2;
                self.data[offset + 3] = color.3;
            },
        }
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }
}


/// Represents the format of a single pixel.
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    /// 1 channel, 8 bits per channel.
    R8 = GlTextureFormat::GlR8 as u32,

    /// 2 channels, 8 bits per channel.
    RG8 = GlTextureFormat::GlR8G8 as u32,

    /// 4 channels, 8 bits per channel.
    RGBA8 = GlTextureFormat::GlR8G8B8A8 as u32,
}

impl PixelFormat {
    pub fn byte_size(&self) -> usize {
        match self {
            PixelFormat::R8 => 1,
            PixelFormat::RG8 => 2,
            PixelFormat::RGBA8 => 4,
        }
    }
}

impl From<PixelFormat> for GlTextureFormat {
    fn from(value: PixelFormat) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}
