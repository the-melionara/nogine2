use nogine2::{colors::{rgba::RGBA32, Color}, graphics::CameraData, math::vector2::{uvec2, vec2}, prelude::init_nogine2, window::{Window, WindowCfg}};

fn main() {
    init_nogine2();
    
    let mut window = Window::new(WindowCfg { title: "Nogine 2", res: uvec2(1280, 720) });
    window.set_vsync(true);

    while window.is_open() {
        window.pre_tick(CameraData { center: vec2::ZERO, extents: vec2(window.aspect_ratio(), 1.0) * 2.0 }, window.res(), RGBA32::BLACK, None);

        window.post_tick();
    }
}
