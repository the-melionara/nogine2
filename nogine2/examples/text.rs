use nogine2::{colors::{rgba::RGBA32, Color}, graphics::{scope::RenderScopeCfgFlags, text::MSDFFont, CameraData, FrameSetup, Graphics}, input::{keyboard::{self, Key}, Input}, math::vector2::{uvec2, vec2}, prelude::init_nogine2, window::{Window, WindowCfg}};

fn main() {
    init_nogine2();
    
    let mut window = Window::new(WindowCfg { title: "Text example", res: uvec2(1280, 720) });

    let font = MSDFFont::new("assets/OpenSans-Light.ttf");

    let mut pos = vec2::ZERO;
    let mut height = 5.0;

    Graphics::set_pixels_per_unit(16.0);
    
    while window.is_open() {
        window.pre_tick(FrameSetup {
            camera: CameraData { center: pos, extents: vec2(window.aspect_ratio(), 1.0) * height },
            target_res: window.res(),
            clear_col: RGBA32::GRAY,
            ..Default::default()
        });

        pos += vec2::from(Input::keyboard().axis2((Key::A, Key::S), (Key::D, Key::W))) * height * window.ts();
        height += Input::mouse().scroll().1;

        // Graphics::draw_rect(vec2::ZERO, 0.0, vec2::one(1.0), RGBA32::GRAY);
        font.draw();

        window.post_tick();
    }
}
