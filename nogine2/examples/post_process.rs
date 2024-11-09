use std::sync::Arc;

use nogine2::{colors::{rgba::RGBA32, Color}, graphics::{defaults::DefaultSubShaders, material::Material, pipeline::{RenderPipeline, RenderStats, SceneData}, shader::{Shader, SubShader, SubShaderType}, texture::{rendertex::RenderTexture, TextureFiltering, TextureSampling, TextureWrapping}, CameraData, Graphics}, input::{keyboard::Key, Input}, math::vector2::{uvec2, vec2}, prelude::init_nogine2, window::{Window, WindowCfg}};

fn main() {
    init_nogine2();
    
    let mut window = Window::new(WindowCfg { title: "Nogine 2 (press enter to toggle post process)", res: uvec2(1280, 720) });
    window.set_vsync(true);
 
    let pipeline = PostProcessPipeline {
        invert_mat: Material::new(Shader::new(
            &DefaultSubShaders::blit_vert(),
            &SubShader::new(FRAG_SRC, SubShaderType::Fragment).unwrap()
        ).unwrap()),
    };

    let mut center = vec2::ZERO;
    let mut pp_enabled = false;
    while window.is_open() {
        center += vec2::from(Input::keyboard().axis2((Key::A, Key::S), (Key::D, Key::W))) * window.ts();
        pp_enabled ^= Input::keyboard().key_pressed(Key::Enter);

        window.pre_tick(CameraData { center, extents: vec2(window.aspect_ratio(), 1.0) * 5.0 }, window.res(), RGBA32::BLACK,
            if pp_enabled { Some(&pipeline) } else { None }
        );

        Graphics::draw_rect(vec2(-2.0, -0.5), 0.0, vec2::ONE, RGBA32::RED);
        Graphics::draw_rect(vec2(-0.5, -0.5), 0.0, vec2::ONE, RGBA32::GREEN);
        Graphics::draw_rect(vec2( 1.0, -0.5), 0.0, vec2::ONE, RGBA32::BLUE);

        dbg!(window.post_tick());
    }
}

const FRAG_SRC: &[u8] = br#"
#version 330 core

#define MAX_TEXTURES 16

layout(location = 0) out vec4 fCol;

in vec4 vTint;
in vec2 vUV;

uniform sampler2D uTextures[MAX_TEXTURES];

void main() {
    fCol = texture(uTextures[0], vUV) * vTint;
    fCol.xyz = 1.0 - fCol.xyz;
}
"#;

struct PostProcessPipeline {
    invert_mat: Arc<Material>,
}

impl RenderPipeline for PostProcessPipeline {
    fn render(&self, target_rt: &RenderTexture, scene_data: SceneData<'_>, clear_col: RGBA32, stats: &mut RenderStats) {
        let src_rt = RenderTexture::new(target_rt.dims(), TextureSampling { filtering: TextureFiltering::Linear, wrapping: TextureWrapping::Clamp });
        src_rt.clear(clear_col);
        scene_data.render_to(&src_rt, stats);

        target_rt.combine_with_material(&src_rt, self.invert_mat.clone(), stats);
    }
}
