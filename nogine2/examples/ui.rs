use nogine2::{colors::{rgba::RGBA32, Color}, graphics::{texture::{Texture2D, TextureFiltering, TextureSampling, TextureWrapping}, ui::Anchor, FrameSetup, Graphics}, math::vector2::{uvec2, vec2}, prelude::init_nogine2, window::{Window, WindowCfg}};

fn main() {
    init_nogine2();
    
    let mut window = Window::new(WindowCfg { title: "Nogine 2", res: uvec2(1280, 720) });
    window.set_vsync(true);
    
    let texture = Texture2D::load("assets/jerma.jpeg", TextureSampling { filtering: TextureFiltering::Nearest, wrapping: TextureWrapping::Repeat }).unwrap();

    while window.is_open() {
        window.pre_tick(FrameSetup {
            target_res: window.fb_size(), ui_res: Some(window.fb_size()), ..Default::default()
        });

        let mut count = 0;
        Graphics::ui(|area| {
            area.draw_texture(Anchor::Center, vec2::ZERO, 0.0, vec2::ONE, RGBA32::WHITE, &texture);
        });
        dbg!(count);

        (window.post_tick());
    }
}
