use std::{marker::PhantomData, num::Wrapping};

use nogine2_core::math::{lerp::CompLerp, rect::Rect, vector2::{uvec2, vec2}};

use crate::{colors::rgba::RGBA32, graphics::{scope::RenderScope, texture::{Texture2D, TextureHandle}, RectSubmitCmd, WHITE_TEX}};

use super::{hash, layout::{horizontal::UIHorizontalLayout, vertical::UIVerticalLayout}, Anchor, UIHash, UIWidget};

/// Main element for UI rendering.
pub struct UIArea<'a> {
    rect: Rect,
    id: UIHash,
    scope: *mut RenderScope,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> UIArea<'a> {
    pub(crate) fn root(res: uvec2, scope: *mut RenderScope) -> Self {
        Self { rect: Rect { start: vec2::ZERO, end: vec2::from(res) }, id: Wrapping(0), scope, _phantom: PhantomData }
    }
    
    pub fn draw_rect(&self, anchor: Anchor, offset: vec2, rot: f32, extents: vec2, color: RGBA32) {
        let scope = unsafe { self.scope.as_mut().unwrap_unchecked() };

        let pivot = anchor.local_pivot();
        let pos = self.rect.start.clerp(self.rect.end, pivot) + offset;
        scope.set_pivot(pivot);
        
        scope.draw_rect(RectSubmitCmd {
            pos, rot, extents,
            tint: [color; 4], texture: WHITE_TEX.get(), uv_rect: Rect::IDENT,
        });
    }

    pub fn draw_texture(&self, anchor: Anchor, offset: vec2, rot: f32, scale: vec2, tint: RGBA32, texture: &Texture2D) {
        return self.draw_texture_adv(anchor, offset, rot, scale, [tint; 4], texture.handle(), Rect::IDENT);
    }
        
    pub fn draw_texture_adv(&self, anchor: Anchor, offset: vec2, rot: f32, scale: vec2, tint: [RGBA32; 4], texture: TextureHandle, uv_rect: Rect) {
        let scope = unsafe { self.scope.as_mut().unwrap_unchecked() };

        let pivot = anchor.local_pivot();
        let pos = self.rect.start.clerp(self.rect.end, pivot) + offset;
        scope.set_pivot(pivot);
        let extents = vec2::from(texture.dims()).scale(uv_rect.size()).scale(scale);
        
        scope.draw_rect(RectSubmitCmd {
            pos, rot, extents,
            tint, texture, uv_rect,
        });
    }

    /// Creates a sub area inside the current one. `name` should be unique among all this sub area's siblings and consistent across frames for it to work properly.
    pub fn named_sub_area<'b, R>(&self, name: &'a str, anchor: Anchor, rect: UIRect, f: impl FnOnce(UIArea<'b>) -> R) -> R where 'a: 'b, Self: 'static {
        self.unique_sub_area(name.as_bytes(), anchor, rect, f)
    }

    /// Creates a sub area inside the current one. `unique` should be unique among all this sub area's siblings and consistent across frames for it to work properly.
    pub fn unique_sub_area<'b, R>(&self, unique_data: &[u8], anchor: Anchor, rect: UIRect, f: impl FnOnce(UIArea<'b>) -> R) -> R where 'a: 'b, Self: 'static {
        let id = hash::fnv1(self.id, unique_data);
        self.sub_area(id, anchor, rect, f)
    }

    /// Creates a sub area inside the current one. `id` should be unique among all this sub area's siblings and consistent across frames for it to work properly.
    pub fn sub_area<'b, R>(&self, id: UIHash, anchor: Anchor, rect: UIRect, f: impl FnOnce(UIArea<'b>) -> R) -> R where 'a: 'b {
        return f(Self {
            rect: rect.to_rect(anchor, self.rect),
            id,
            scope: self.scope, _phantom: PhantomData
        });
    }

    /// Creates a sub area inside the current one with metadata. `name` should be unique among all this sub area's siblings and consistent across frames for it to work properly.
    pub fn named_sub_area_with_metadata<'b, M, R>(&self, name: &'a str, anchor: Anchor, rect: UIRect, meta: M, f: impl FnOnce(UIArea<'b>, M) -> R) -> R where 'a: 'b {
        self.unique_sub_area_with_metadata(name.as_bytes(), anchor, rect, meta, f)
    }

    /// Creates a sub area inside the current one with metadata. `unique` should be unique among all this sub area's siblings and consistent across frames for it to work properly.
    pub fn unique_sub_area_with_metadata<'b, M, R>(&self, unique: &[u8], anchor: Anchor, rect: UIRect, meta: M, f: impl FnOnce(UIArea<'b>, M) -> R) -> R where 'a: 'b {
        let id = hash::fnv1(self.id, unique);
        self.sub_area_with_metadata(id, anchor, rect, meta, f)
    }

    /// Creates a sub area inside the current one with metadata. `id` should be unique among all this sub area's siblings and consistent across frames for it to work properly.
    pub fn sub_area_with_metadata<'b, M, R>(&self, id: UIHash, anchor: Anchor, rect: UIRect, meta: M, f: impl FnOnce(UIArea<'b>, M) -> R) -> R where 'a: 'b {
        return f(Self {
            rect: rect.to_rect(anchor, self.rect),
            id,
            scope: self.scope, _phantom: PhantomData
        }, meta);
    }

    pub fn attach<W: UIWidget<'a>>(&self, widget: W) -> W::RunRet {
        let id = hash::fnv1(self.id, widget.unique_data());
        self.attach_with_id(widget, id)
    }

    pub fn attach_with_id<W: UIWidget<'a>>(&self, mut widget: W, id: UIHash) -> W::RunRet {
        widget.set_id(id);
        return widget.run(&Self {
            rect: self.rect,
            id,
            scope: self.scope, _phantom: PhantomData
        });
    }

    pub fn vertical_layout<'b>(&self, name: &'a str, count: usize, f: impl FnMut(UIArea<'b>, usize) + 'a) where 'a: 'b {
        self.attach(UIVerticalLayout::new(name, count, f))
    }
    
    pub fn horizontal_layout<'b>(&self, name: &'a str, count: usize, f: impl FnMut(UIArea<'b>, usize) + 'a) where 'a: 'b {
        self.attach(UIHorizontalLayout::new(name, count, f))
    }

    pub fn size(&self) -> vec2 {
        self.rect.size()
    }

    pub fn center(&self) -> vec2 {
        self.rect.center()
    }

    pub fn rect(&self) -> Rect {
        self.rect
    }
}


#[derive(Debug, Clone, Copy)]
pub struct UIRect {
    pub offset: vec2,
    pub size: vec2,
}

impl UIRect {
    pub fn to_rect(&self, anchor: Anchor, parent_rect: Rect) -> Rect {
        let local_pivot = anchor.local_pivot();
        let start = parent_rect.start.clerp(parent_rect.end, local_pivot) - self.size.scale(local_pivot) + self.offset;
        let end = start + self.size;
        return Rect { start, end };
    }
}
