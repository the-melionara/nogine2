use nogine2::{colors::{rgba::RGBA32, Color}, graphics::{blending::BlendingMode, texture::{sprite::SpriteAtlas, Texture2D, TextureFiltering, TextureSampling, TextureWrapping}, CameraData, FrameSetup, Graphics}, input::{keyboard::Key, Input}, math::{rect::IRect, vector2::{ivec2, uvec2, vec2}}, prelude::init_nogine2, window::{Window, WindowCfg}};

fn main() {
    init_nogine2();
    
    let mut window = Window::new(WindowCfg { title: "Nogine 2", res: uvec2(1280, 720) });
    window.set_vsync(true);

    let texture = Texture2D::load("assets/tileset.png", TextureSampling { filtering: TextureFiltering::Nearest, wrapping: TextureWrapping::Clamp }).unwrap();
    let atlas = SpriteAtlas::new(texture, uvec2(16, 16));

    Graphics::set_pixels_per_unit(16.0);

    while window.is_open() {
        window.pre_tick(FrameSetup {
            camera: CameraData { center: vec2::ZERO, extents: vec2(window.aspect_ratio(), 1.0) * 5.0 },
            target_res: window.res(), ..Default::default()
        });

        Graphics::draw_sprite(vec2(-2.3, 0.1), 0.0, vec2::ONE, &atlas.get(IRect { start: ivec2::ZERO, end: ivec2::ONE }));
        Graphics::draw_sprite(vec2(-1.1, 0.1), 0.0, vec2::ONE, &atlas.get(IRect { start: ivec2(1, 0), end: ivec2(2, 1) }));
        Graphics::draw_sprite(vec2(0.1, 0.1), 0.0, vec2::ONE, &atlas.get(IRect { start: ivec2(2, 0), end: ivec2(3, 1) }));
        Graphics::draw_sprite(vec2(1.3, 0.1), 0.0, vec2::ONE, &atlas.get(IRect { start: ivec2(3, 0), end: ivec2(4, 1) }));

        Graphics::draw_sprite(vec2(-1.1, -1.2), 0.0, vec2::ONE, &atlas.get(IRect { start: ivec2(0, 1), end: ivec2(1, 2) }));
        Graphics::draw_sprite(vec2(0.1, -1.2), 0.0, vec2::ONE, &atlas.get(IRect { start: ivec2(1, 1), end: ivec2(2, 2) }));

        dbg!(window.post_tick());
    }
}
