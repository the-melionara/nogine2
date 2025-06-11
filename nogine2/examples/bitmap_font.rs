use std::fs::File;

use nogine2::{colors::{rgba::RGBA32, Color}, graphics::{text::{font::{BitmapFont, TextStyle}, TextCfg}, texture::{sprite::SpriteAtlas, Texture2D, TextureFiltering, TextureSampling, TextureWrapping}, CameraData, FrameSetup, Graphics}, math::{rect::Rect, vector2::{uvec2, vec2}}, prelude::init_nogine2, unwrap_res, window::{Window, WindowCfg}};

fn main() {
    init_nogine2();
    let mut window = Window::new(WindowCfg { title: "Bitmap Font", res: uvec2(1280, 720) });

    let texture = unwrap_res!(Texture2D::load("assets/nice_text.png",
        TextureSampling {
            filtering: TextureFiltering::Nearest,
            wrapping: TextureWrapping::Clamp,
        }
    ));
    let atlas = SpriteAtlas::new(texture, uvec2(10, 9));
    let font = BitmapFont::new(
        atlas,
        "0123456789.,:;'()[]{}<>?!¿¡_*+-=/#%@~ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyzÁÉÍÓÚÚáéíóúüÑñ",
        false, 3.0,
    );

    Graphics::set_pixels_per_unit(32.0);
    
    let mut pos = vec2(0.0,0.0);

    while window.is_open() {
        pos += vec2(10.0 * window.ts(),0.0);
        window.pre_tick(FrameSetup {
            camera: CameraData {
                center: vec2::ZERO + pos,
                extents: vec2(window.aspect_ratio() * 60.0, 60.0)
            },
            target_res: window.res(),
            clear_col: RGBA32::BLACK,
            ..Default::default()
        });

        Graphics::draw_text(
            TextCfg { bounds: Rect::IDENT, font_size: 1.0, font: &font },
            "DELTARUNE TOMORROW"
        );

        window.post_tick();
    }
}
