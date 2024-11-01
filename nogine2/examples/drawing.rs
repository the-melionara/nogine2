use nogine2::{colors::{rgba::RGBA32, Color}, graphics::{texture::{pixels::Pixels, Texture2D, TextureFiltering, TextureSampling, TextureWrapping}, CameraData, Graphics}, input::{keyboard::Key, Input}, math::vector2::{uvec2, vec2}, prelude::init_nogine2, window::{Window, WindowCfg}};

fn main() {
    init_nogine2();
    
    let mut window = Window::new(WindowCfg { title: "Nogine 2", res: uvec2(1280, 720) });
    window.set_vsync(true);

    let texture = Texture2D::load("assets/jerma.jpeg", TextureSampling { filtering: TextureFiltering::Nearest, wrapping: TextureWrapping::Repeat }).unwrap();

    Graphics::set_pixels_per_unit(350.0);

    let mut center = vec2::ZERO;
    while window.is_open() {
        center += vec2::from(Input::keyboard().axis2((Key::A, Key::S), (Key::D, Key::W))) * window.ts();
        window.pre_tick(CameraData { center, extents: vec2(window.aspect_ratio(), 1.0) * 5.0 }, window.res(), RGBA32::BLACK, None);

        Graphics::draw_rect(vec2(-1.0, -1.0), 0.0, vec2::one(2.0), RGBA32::WHITE);

        Graphics::draw_rect(vec2(-1.5, -0.75), 0.0, vec2::one(1.5), RGBA32::YELLOW);
        Graphics::draw_rect(vec2( 0.0, -0.75), 0.0, vec2::one(1.5), RGBA32::CYAN);

        Graphics::draw_texture(vec2(-2.0, -0.5), 0.0, vec2::ONE, RGBA32::RED, &texture);
        Graphics::draw_texture(vec2(-0.5, -0.5), 0.0, vec2::ONE, RGBA32::GREEN, &texture);
        Graphics::draw_texture(vec2( 1.0, -0.5), 0.0, vec2::ONE, RGBA32::BLUE, &texture);

        dbg!(window.post_tick());
    }
}
