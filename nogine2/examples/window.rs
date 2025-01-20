use nogine2::{graphics::FrameSetup, math::vector2::uvec2, prelude::init_nogine2, window::{Window, WindowCfg}};

fn main() {
    init_nogine2();
    
    let mut window = Window::new(WindowCfg { title: "Nogine 2", res: uvec2(1280, 720) });
    window.set_vsync(true);

    while window.is_open() {
        window.pre_tick(FrameSetup { target_res: window.fb_size(), ..Default::default() });

        window.post_tick();
    }
}
