use nogine2::{colors::{rgba::RGBA32, Color}, graphics::{CameraData, FrameSetup, Graphics}, math::vector2::{uvec2, vec2}, prelude::init_nogine2, window::{Window, WindowCfg}};

fn main() {
    init_nogine2();
    
    let mut window = Window::new(WindowCfg { title: "Nogine 2", res: uvec2(1280, 720) });
    window.set_vsync(true);

    while window.is_open() {
        window.pre_tick(FrameSetup {
            camera: CameraData { center: vec2::ZERO, extents: vec2(window.aspect_ratio(), 1.0) * 5.0 },
            target_res: window.res(), ..Default::default()
        });

        Graphics::set_pivot(vec2::one(0.5));
        Graphics::draw_rect(vec2::ZERO, window.time().as_secs_f32(), vec2(0.5, 3.0), RGBA32::WHITE);
        Graphics::set_pivot(vec2::ZERO);

        window.post_tick();
    }
}
