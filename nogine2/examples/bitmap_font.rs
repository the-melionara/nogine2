use std::{any::Any, fs::File};

use nogine2::{colors::{rgba::RGBA32, Color}, graphics::{gfx::screen_to_world_pos, text::{align::HorTextAlign, font::{BitmapFont, FontCfg, Measure, TextStyle}, TextCfg}, texture::{sprite::SpriteAtlas, Texture2D, TextureFiltering, TextureSampling, TextureWrapping}, CameraData, FrameSetup, Graphics}, input::{keyboard::Key, mouse::Button, Input}, log_info, math::{rect::Rect, vector2::{uvec2, vec2}}, prelude::init_nogine2, unwrap_res, window::{Window, WindowCfg}};

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
        FontCfg {
            monospace: false,
            space_width: Measure::Percent(0.5),
            char_separation: Measure::Percent(1.0 / 9.0),
        }
    );

    Graphics::set_pixels_per_unit(16.0);

    let mut text_pos = vec2::ZERO;
    let mut width = 6.0;
    let mut ruler_origin = None;
    
    while window.is_open() {
        window.pre_tick(FrameSetup {
            camera: CameraData {
                center: vec2::ZERO,
                extents: vec2(window.aspect_ratio() * 6.0, 6.0)
            },
            target_res: window.res(),
            clear_col: RGBA32::BLACK,
            ..Default::default()
        });

        text_pos += vec2::from(Input::keyboard().axis2(
            (Key::Left, Key::Down),
            (Key::Right, Key::Up))
        ) * window.ts();

        width += Input::keyboard().axis1(Key::A, Key::D) as f32 * window.ts();
        
        Graphics::draw_text(
            TextCfg {
                origin: text_pos,
                extents: vec2(width, 2.0),
                rot: 0.0,
                font_size: 9.0,
                font: &font,
                scale: vec2::ONE,
                hor_alignment: HorTextAlign::Left,
                word_wrap: true,
            },

            "DELTARUNE TOMORROW REAL NO FAKE 1 LINK MEDIAFIRE"
        );

        match ruler_origin {
            Some(origin) => {
                let target = screen_to_world_pos(Input::mouse().pos());
                Graphics::draw_line(origin, target, [RGBA32::RED; 2]);

                log_info!("Ruler: {}", origin.dist_to(target));

                if Input::mouse().button_released(Button::Left) {
                    ruler_origin = None;
                }
            },
            None => {
                if Input::mouse().button_pressed(Button::Left) {
                    ruler_origin = Some(screen_to_world_pos(Input::mouse().pos()));
                }
            },
        }

        dbg!(window.post_tick());
    }
}
