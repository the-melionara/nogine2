use nogine2::{graphics::FrameSetup, input::{keyboard::Key, Input}, log_info, math::vector2::uvec2, prelude::init_nogine2, window::{Window, WindowCfg}};

fn main() {
    init_nogine2();
    
    let mut window = Window::new(WindowCfg { title: "Nogine 2", res: uvec2(1280, 720) });
    window.set_vsync(true);

    while window.is_open() {
        window.pre_tick(FrameSetup { target_res: window.res(), ..Default::default() });

        let keyboard = Input::keyboard();
        if keyboard.key_pressed(Key::Enter) {
            log_info!("Enter Pressed");
        }
        if keyboard.key_released(Key::Enter) {
            log_info!("Enter Released");
        }

        window.post_tick();
    }
}
