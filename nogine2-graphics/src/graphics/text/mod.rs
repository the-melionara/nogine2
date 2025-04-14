use nogine2_core::math::{lerp::Lerp, vector2::vec2};

use crate::colors::{rgba::RGBA32, Color};

use super::{texture::Texture2D, Graphics};

pub struct MSDFFont {
    image: Texture2D,
}

impl MSDFFont {
    #[cfg(feature = "font-loading")]
    pub fn new(path: impl AsRef<std::path::Path>) -> Self {
        use std::{fs::File, io::{BufReader, Read}};

        let mut buf = Vec::new();
        let mut reader = BufReader::new(File::open(path).unwrap());
        reader.read_to_end(&mut buf).unwrap();

        return Self::from_byte_slice(&buf);
    }

    #[cfg(feature = "font-loading")]
    pub fn from_byte_slice(data: &[u8]) -> Self {
        use nogine2_core::math::vector2::uvec2;
        use nogine2_msdfgen::generator::{MSDFGenConfig, MSDFGenerator};

        use crate::graphics::texture::{pixels::{PixelFormat, Pixels}, Texture2D, TextureFiltering, TextureSampling, TextureWrapping};

        let generator = MSDFGenerator::new(data, MSDFGenConfig { pixels_per_em: 32, field_range: 1.0 });
        let gen_data = generator.run().unwrap();

        let pixels = Pixels::new(gen_data.pixels, uvec2(gen_data.width, gen_data.height), PixelFormat::R8);
        let image = Texture2D::new(pixels, TextureSampling { filtering: TextureFiltering::Linear, wrapping: TextureWrapping::Clamp });

        return Self { image };
    }

    pub fn draw(&self) {
        Graphics::draw_texture(vec2::ZERO, 0.0, vec2::ONE, RGBA32::WHITE, &self.image);
    }
}
