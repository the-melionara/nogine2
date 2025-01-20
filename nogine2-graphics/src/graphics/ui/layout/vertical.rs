use std::num::Wrapping;

use nogine2_core::math::vector2::vec2;

use crate::graphics::ui::{area::{UIArea, UIRect}, Anchor, UIHash, UIWidget};

/// A widget to create vertical layouts.
pub struct UIVerticalLayout<'a, F> {
    name: &'a str,
    id: UIHash,
    func: F,
    count: usize,
    separation: f32,
}

impl<'a: 'b, 'b, F> UIVerticalLayout<'a, F> where F: FnMut(UIArea<'b>, usize) + 'a {
    pub fn new(name: &'a str, count: usize, func: F) -> Self {
        Self { name, id: Wrapping(0), func, count, separation: 0.0 }
    }

    pub fn with_separation(name: &'a str, count: usize, separation: f32, func: F) -> Self {
        Self { name, id: Wrapping(0), count, separation, func }
    }
}

impl<'a: 'b, 'b, F> UIWidget<'a> for UIVerticalLayout<'a, F> where F: FnMut(UIArea<'b>, usize) + 'a {
    fn unique_data(&self) -> &[u8] {
        self.name.as_bytes()
    }

    fn set_id(&mut self, id: UIHash) {
        self.id = id;
    }

    fn id(&self) -> UIHash {
        self.id
    }

    type RunRet = ();

    fn run(mut self, parent: &UIArea<'a>) -> Self::RunRet {
        let separation_count = self.count.max(1) - 1;
        let size = (parent.size() - vec2(0.0, self.separation * separation_count as f32)).inv_scale(vec2(1.0, self.count as f32));
        for i in 0..self.count {
            let rect = UIRect { offset: vec2(0.0, (size.1 + self.separation) * i as f32), size };
            parent.unique_sub_area_with_metadata(&i.to_le_bytes(), Anchor::LeftUp, rect, i, |area, i| (self.func)(area, i));
        }
    }
}
