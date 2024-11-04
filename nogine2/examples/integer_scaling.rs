use nogine2::{colors::{rgba::RGBA32, Color}, graphics::{gfx::integer_scaling_blit, pipeline::{RenderPipeline, RenderStats, SceneData}, texture::{rendertex::RenderTexture, TextureFiltering, TextureSampling, TextureWrapping}, CameraData, Graphics}, input::{keyboard::Key, Input}, math::vector2::{uvec2, vec2}, prelude::init_nogine2, window::{Window, WindowCfg}};

struct CustomPipeline;
impl RenderPipeline for CustomPipeline {
    fn render(&self, target_rt: &RenderTexture, scene_data: SceneData<'_>, clear_col: RGBA32, stats: &mut RenderStats) {
        let downscale_rt = RenderTexture::new(uvec2(320, 180), TextureSampling { filtering: TextureFiltering::Nearest, wrapping: TextureWrapping::Clamp });
        downscale_rt.clear(clear_col);
        scene_data.render_to(&downscale_rt, stats);

        target_rt.clear(RGBA32::BLACK);
        integer_scaling_blit(&downscale_rt, target_rt, stats);
    }
}

fn main() {
    init_nogine2();
    
    let mut window = Window::new(WindowCfg { title: "Nogine 2", res: uvec2(1280, 720) });
    window.set_vsync(true);

    let mut pos = vec2::ZERO;
    let pipeline = CustomPipeline;
    while window.is_open() {
        window.pre_tick(CameraData { center: vec2::ZERO, extents: vec2(16.0, 9.0) }, uvec2(320, 180), RGBA32(0.1, 0.2, 0.3, 1.0), Some(&pipeline));

        pos += vec2::from(Input::keyboard().axis2((Key::A, Key::S), (Key::D, Key::W))) * window.ts();
        Graphics::draw_rect(pos, 0.0, vec2::ONE, RGBA32::WHITE);

        toggle_fullscreen(&mut window);
        window.post_tick();
    }
}

fn toggle_fullscreen(window: &mut Window) {
    let keyboard = Input::keyboard();
    if keyboard.key(Key::LeftAlt) && keyboard.key_pressed(Key::Enter) {
        window.toggle_fullscreen();
    }
}
