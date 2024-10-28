use nogine2::{colors::{rgba::RGBA32, Color}, graphics::{BeginRenderCmd, CameraData}, math::vector2::uvec2, prelude::init_nogine2, window::{Window, WindowCfg}};

fn main() {
    init_nogine2();
    
    let mut window = Window::new(WindowCfg { title: "Nogine 2", res: uvec2(1280, 720) });
    window.set_vsync(true);

    while window.is_open() {
        window.pre_tick(BeginRenderCmd::new(CameraData::default(), window.res(), RGBA32::BLACK));

        window.post_tick();
    }
}
