use nogine2::{math::vector2::uvec2, window::{Window, WindowCfg}};

fn main() {
    let mut window = Window::new(WindowCfg { title: "Nogine 2", res: uvec2(1280, 720) });
    window.set_vsync(true);

    while window.is_open() {
        window.pre_tick();

        window.post_tick();
    }
}
