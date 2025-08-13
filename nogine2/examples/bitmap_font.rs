use std::{any::Any, fs::File, str::Split};

use nogine2::{colors::{rgba::RGBA32, Color}, graphics::{gfx::screen_to_world_pos, scope::RenderScopeCfgFlags, text::{align::{HorTextAlign, VerTextAlign}, font::{bitmap::BitmapFont, FontCfg, Measure, TextStyle}, rich::{CharQuad, RichTextContext, RichTextFunction}, TextCfg}, texture::{sprite::SpriteAtlas, Texture2D, TextureFiltering, TextureSampling, TextureWrapping}, ui::Anchor, CameraData, FrameSetup, Graphics}, input::{keyboard::Key, mouse::Button, Input}, log_info, math::{rect::Rect, vector2::{uvec2, vec2}}, prelude::init_nogine2, unwrap_res, window::{Window, WindowCfg}};

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
    
    Graphics::set_rich_text(true);
    Graphics::set_word_wrap(true);
    Graphics::set_text_hor_alignment(HorTextAlign::Center);
    Graphics::set_text_ver_alignment(VerTextAlign::Center);
    Graphics::set_font_size(9.0);

    Graphics::enable_cfg(RenderScopeCfgFlags::POSITIVE_Y_IS_DOWN);

    while window.is_open() {
        window.pre_tick(FrameSetup {
            camera: CameraData {
                center: vec2::ZERO,
                extents: vec2(window.aspect_ratio() * 6.0, 6.0)
            },
            target_res: window.res(),
            clear_col: RGBA32::BLACK,
            ui_res: Some(window.res()),
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

        Graphics::ui(|ui| {
            ui.horizontal_layout("hor", 2, |ui, i| {
                const TEXT: [&str; 2] = ["A tale of love and hate", "A tale of light and dark"];
                const COLS: [RGBA32; 2] = [RGBA32::DARK_RED, RGBA32::DARK_BLUE];

                ui.draw_rect(Anchor::Center, vec2::ZERO, 0.0, ui.size(), COLS[i]);

                ui.set_rich_text(true);
                ui.set_word_wrap(true);
                ui.set_text_hor_alignment(HorTextAlign::Center);
                ui.set_text_ver_alignment(VerTextAlign::Center);
                ui.set_font_size(90.0);

                ui.draw_text(Anchor::Center, vec2::ZERO, 0.0, ui.size(), TEXT[i], &font);
            });
        });

        // Graphics::set_pivot(vec2::one(0.5));
        // Graphics::draw_text(
        //     text_pos,
        //     0.0,
        //     extents,
        //     "human, i <wave>remember you're <red>genocides</red> eeeeee</wave> eeee",
        //     &font
        // );

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
