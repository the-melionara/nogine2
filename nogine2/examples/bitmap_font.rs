use std::{any::Any, fs::File, str::Split};

use nogine2::{colors::{rgba::RGBA32, Color}, graphics::{gfx::screen_to_world_pos, text::{align::{HorTextAlign, VerTextAlign}, font::{bitmap::BitmapFont, FontCfg, Measure, TextStyle}, rich::{CharQuad, RichTextContext, RichTextFunction}, TextCfg}, texture::{sprite::SpriteAtlas, Texture2D, TextureFiltering, TextureSampling, TextureWrapping}, CameraData, FrameSetup, Graphics}, input::{keyboard::Key, mouse::Button, Input}, log_info, math::{rect::Rect, vector2::{uvec2, vec2}}, prelude::init_nogine2, unwrap_res, window::{Window, WindowCfg}};

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
    let mut font = BitmapFont::new(
        atlas,
        "0123456789.,:;'()[]{}<>?!¿¡_*+-=/#%@~ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyzÁÉÍÓÚÚáéíóúüÑñ",
        FontCfg {
            monospace: false,
            space_width: Measure::Percent(0.5),
            char_separation: Measure::Percent(1.0 / 9.0),
        }
    );
    font.add_rich_function(Box::new(RTFRed));
    font.add_rich_function(Box::new(RTFWave));

    Graphics::set_pixels_per_unit(16.0);

    let mut text_pos = vec2::ZERO;
    let mut extents = vec2(6.0, 2.0);
    
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

        extents += vec2::from(Input::keyboard().axis2(
            (Key::A, Key::S),
            (Key::D, Key::W)
        )) * window.ts();
        
        Graphics::draw_text(
            TextCfg {
                origin: text_pos,
                extents,
                rot: 0.0,
                font_size: 9.0,
                font: &font,
                scale: vec2::ONE,
                hor_alignment: HorTextAlign::Center,
                ver_alignment: VerTextAlign::Center,
                word_wrap: true,
                rich_text: true,
            },

            "human, i <wave>remember you're <red>genocides</red> eeeeee</wave> eeee"
        );

        dbg!(window.post_tick());
    }
}

struct RTFRed;
impl RichTextFunction for RTFRed {
    fn get_tag_name(&self) -> &'static str {
        "red"
    }

    fn draw(
        &self,
        _args: Split<'_, char>,
        _in_quads: &[CharQuad],
        out_quads: &mut Vec<CharQuad>,
        _ctx: &RichTextContext
    ) {
        for q in out_quads {
            q.lu.color = RGBA32::RED;
            q.ld.color = RGBA32::RED;
            q.ru.color = RGBA32::RED;
            q.rd.color = RGBA32::RED;
        }
    }
}

struct RTFWave;
impl RichTextFunction for RTFWave {
    fn get_tag_name(&self) -> &'static str {
        "wave"
    }

    fn draw(
        &self,
        _args: Split<'_, char>,
        _in_quads: &[CharQuad],
        out_quads: &mut Vec<CharQuad>,
        ctx: &RichTextContext
    ) {
        let y_offset = (ctx.time * 4.0 + ctx.index as f32 * 0.5).sin() * 0.05;
        
        for q in out_quads {
            q.lu.pos += vec2::up(y_offset);
            q.ld.pos += vec2::up(y_offset);
            q.ru.pos += vec2::up(y_offset);
            q.rd.pos += vec2::up(y_offset);
        }
    }
}
