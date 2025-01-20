use nogine2_core::math::vector2::{uvec2, vec2};

use crate::{colors::rgba::RGBA32, graphics::{pipeline::{DefaultPipeline, RenderPipeline, RenderStats, SceneData}, texture::rendertex::RenderTexture, ui::area::UIArea, CameraData}};

use super::{RenderScope, RenderScopeCfgFlags};

static DEFAULT_PIPELINE: DefaultPipeline = DefaultPipeline;

pub struct UIScope {
    inner: RenderScope,
}

impl UIScope {
    pub const fn new() -> Self {
        let mut inner = RenderScope::new();
        inner.set_cfg(RenderScopeCfgFlags::DEFAULT_UI);
        return Self { inner };
    }

    /// Makes this scope the target for all render commands.
    pub fn run<'b, 'a: 'b, R>(&'a mut self, rt: &RenderTexture, pipeline: Option<&dyn RenderPipeline>, f: impl FnOnce(UIArea<'b>) -> R) -> (R, RenderStats) {
        let pipeline = if let Some(pipeline) = pipeline {
            unsafe { std::mem::transmute::<_, *const dyn RenderPipeline>(pipeline) } // Hack to stop misdiagnosis from rust (?)
        } else {
            &DEFAULT_PIPELINE as *const dyn RenderPipeline
        };

        self.begin_render(rt.dims(), pipeline);
        let res = f(UIArea::root(rt.dims(), &mut self.inner));
        return (res, self.end_render(rt));
    }

    /// Asummes begin_render has already been called
    pub(in crate::graphics) fn run_internal<'b, 'a: 'b, R>(&'a mut self, f: impl FnOnce(UIArea<'b>) -> R) -> R {
        f(UIArea::root(self.inner.target_res(), &mut self.inner))
    }

    pub(crate) fn begin_render(&mut self, res: uvec2, pipeline: *const dyn RenderPipeline) {
        self.inner.begin_render(CameraData { center: vec2::from(res).scale(vec2(0.5, 0.5)), extents: vec2::from(res) }, res, RGBA32::CLEAR, pipeline);
    }

    pub(crate) fn end_render(&mut self, rt: &RenderTexture) -> RenderStats {
        self.inner.end_render(rt, true, None)
    }

    pub(crate) fn get_scene_data(&self) -> SceneData<'_> {
        self.inner.get_scene_data()
    }
}
