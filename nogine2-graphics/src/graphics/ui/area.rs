use std::{marker::PhantomData, num::Wrapping};

use nogine2_core::math::{lerp::CompLerp, rect::Rect, vector2::{uvec2, vec2}};

use crate::{colors::{rgba::RGBA32, Color}, graphics::{scope::{NinePatchSubmitCmd, RenderScope}, text::{align::{HorTextAlign, VerTextAlign}, font::Font, TextCfg}, texture::{sprite::Sprite, Texture2D, TextureHandle}, RectSubmitCmd, WHITE_TEX}};

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
        Self {
            rect: Rect { start: vec2::ZERO, end: vec2::from(res) },
            id: Wrapping(0),
            scope,
            _phantom: PhantomData
        }
    }
    
    pub fn draw_rect(
        &self,
        anchor: Anchor,
        offset: vec2,
        rot: f32,
        extents: vec2,
        color: RGBA32
    ) {
        let scope = unsafe { self.scope.as_mut().unwrap_unchecked() };

        let pivot = anchor.local_pivot();
        let pos = self.rect.start.clerp(self.rect.end, pivot) + offset;
        scope.set_pivot(pivot);
        
        scope.draw_rect(RectSubmitCmd {
            pos, rot, extents,
            tint: [color; 4], texture: WHITE_TEX.get(), uv_rect: Rect::IDENT,
        });
    }

    pub fn draw_rect_ext(
        &self,
        anchor: Anchor,
        offset: vec2,
        rot: f32,
        extents: vec2,
        colors: [RGBA32; 4]
    ) {
        let scope = unsafe { self.scope.as_mut().unwrap_unchecked() };

        let pivot = anchor.local_pivot();
        let pos = self.rect.start.clerp(self.rect.end, pivot) + offset;
        scope.set_pivot(pivot);
        
        scope.draw_rect(RectSubmitCmd {
            pos, rot, extents,
            tint: colors, texture: WHITE_TEX.get(), uv_rect: Rect::IDENT,
        });
    }

    pub fn draw_texture(
        &self,
        anchor: Anchor,
        offset: vec2,
        rot: f32,
        scale: vec2,
        tint: RGBA32,
        texture: &Texture2D
    ) {
        return self.draw_texture_adv(
            anchor,
            offset,
            rot,
            scale,
            [tint; 4],
            texture.handle(),
            Rect::IDENT
        );
    }
    
    pub fn draw_sprite(
        &self,
        anchor: Anchor,
        offset: vec2,
        rot: f32,
        scale: vec2,
        sprite: &Sprite
    ) {
        return self.draw_texture_adv(anchor,
            offset,
            rot,
            scale,
            [RGBA32::WHITE; 4],
            sprite.handle().clone(),
            sprite.uv_rect()
        );
    }
        
    pub fn draw_texture_adv(
        &self,
        anchor: Anchor,
        offset: vec2,
        rot: f32,
        scale: vec2,
        tint: [RGBA32; 4],
        texture: TextureHandle,
        uv_rect: Rect
    ) {
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

    pub fn draw_9_patch(
        &self,
        anchor: Anchor,
        offset: vec2,
        rot: f32,
        extents: vec2,
        sprite: &Sprite
    ) {
        let scope = unsafe { self.scope.as_mut().unwrap_unchecked() };

        let pivot = anchor.local_pivot();
        let pos = self.rect.start.clerp(self.rect.end, pivot) + offset;
        scope.set_pivot(pivot);
        
        scope.draw_9_patch(NinePatchSubmitCmd {
            pos,
            rot,
            extents,
            tint: RGBA32::WHITE,
            sprite:sprite.clone(),
            corner_scaling: 1.0,
        });
    }

    pub fn draw_9_patch_ext(
        &self,
        anchor: Anchor,
        offset: vec2,
        rot: f32,
        extents: vec2,
        tint: RGBA32,
        sprite: &Sprite,
        corner_scaling: f32
    ) {
        let scope = unsafe { self.scope.as_mut().unwrap_unchecked() };

        let pivot = anchor.local_pivot();
        let pos = self.rect.start.clerp(self.rect.end, pivot) + offset;
        scope.set_pivot(pivot);
        
        scope.draw_9_patch(NinePatchSubmitCmd {
            pos,
            rot,
            extents,
            tint,
            sprite:sprite.clone(),
            corner_scaling,
        });
    }

    pub fn draw_text(
        &self,
        anchor: Anchor,
        offset: vec2,
        rot: f32,
        extents: vec2,
        text: &str,
        font: &dyn Font
    ) {
        let scope = unsafe { self.scope.as_mut().unwrap_unchecked() };

        let pivot = anchor.local_pivot();
        let pos = self.rect.start.clerp(self.rect.end, pivot) + offset;
        scope.set_pivot(pivot);

        scope.draw_text(pos, rot, extents, text, font);
    }

    pub fn draw_text_stateless(&self, anchor: Anchor, mut cfg: TextCfg, text: &str) {
        let scope = unsafe { self.scope.as_mut().unwrap_unchecked() };

        let pivot = anchor.local_pivot();
        let pos = self.rect.start.clerp(self.rect.end, pivot) + cfg.origin;
        scope.set_pivot(pivot);

        cfg.origin = pos;
        scope.draw_text_stateless(cfg, text);
    }


    /// Creates a sub area inside the current one. `name` should be unique among all this sub area's siblings and consistent across frames for it to work properly.
    pub fn named_sub_area<'b, R>(
        &self,
        name: &'a str,
        anchor: Anchor,
        rect: UIRect,
        f: impl FnOnce(UIArea<'b>) -> R
    ) -> R where 'a: 'b, Self: 'static {
        self.unique_sub_area(name.as_bytes(), anchor, rect, f)
    }

    /// Creates a sub area inside the current one. `unique` should be unique among all this sub area's siblings and consistent across frames for it to work properly.
    pub fn unique_sub_area<'b, R>(
        &self,
        unique_data: &[u8],
        anchor: Anchor,
        rect: UIRect,
        f: impl FnOnce(UIArea<'b>) -> R
    ) -> R where 'a: 'b, Self: 'static {
        let id = hash::fnv1(self.id, unique_data);
        self.sub_area(id, anchor, rect, f)
    }

    /// Creates a sub area inside the current one. `id` should be unique among all this sub area's siblings and consistent across frames for it to work properly.
    pub fn sub_area<'b, R>(
        &self,
        id: UIHash,
        anchor: Anchor,
        rect: UIRect,
        f: impl FnOnce(UIArea<'b>) -> R
    ) -> R where 'a: 'b {
        return f(Self {
            rect: rect.to_rect(anchor, self.rect),
            id,
            scope: self.scope,
            _phantom: PhantomData
        });
    }

    /// Creates a sub area inside the current one with metadata. `name` should be unique among all this sub area's siblings and consistent across frames for it to work properly.
    pub fn named_sub_area_with_metadata<'b, M, R>(
        &self,
        name: &'a str,
        anchor: Anchor,
        rect: UIRect,
        meta: M,
        f: impl FnOnce(UIArea<'b>, M) -> R
    ) -> R where 'a: 'b {
        self.unique_sub_area_with_metadata(name.as_bytes(), anchor, rect, meta, f)
    }

    /// Creates a sub area inside the current one with metadata. `unique` should be unique among all this sub area's siblings and consistent across frames for it to work properly.
    pub fn unique_sub_area_with_metadata<'b, M, R>(
        &self,
        unique: &[u8],
        anchor: Anchor,
        rect: UIRect,
        meta: M,
        f: impl FnOnce(UIArea<'b>, M) -> R
    ) -> R where 'a: 'b {
        let id = hash::fnv1(self.id, unique);
        self.sub_area_with_metadata(id, anchor, rect, meta, f)
    }

    /// Creates a sub area inside the current one with metadata. `id` should be unique among all this sub area's siblings and consistent across frames for it to work properly.
    pub fn sub_area_with_metadata<'b, M, R>(
        &self,
        id: UIHash,
        anchor: Anchor,
        rect: UIRect,
        meta: M,
        f: impl FnOnce(UIArea<'b>, M) -> R
    ) -> R where 'a: 'b {
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

    pub fn vertical_layout<'b>(
        &self,
        name: &'a str,
        count: usize,
        f: impl FnMut(UIArea<'b>, usize) + 'a
    ) where 'a: 'b {
        self.attach(UIVerticalLayout::new(name, count, f))
    }
    
    pub fn horizontal_layout<'b>(
        &self,
        name: &'a str,
        count: usize,
        f: impl FnMut(UIArea<'b>, usize) + 'a
    ) where 'a: 'b {
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

    /// Returns the font size.
    pub fn font_size(&self) -> f32 {
        let scope = unsafe { self.scope.as_ref().unwrap_unchecked() };
        return scope.font_size();
    }

    /// Sets the font size.
    pub fn set_font_size(&self, font_size: f32) {
        let scope = unsafe { self.scope.as_mut().unwrap_unchecked() };
        scope.set_font_size(font_size);
    }

    /// Returns the font col.
    pub fn font_col(&self) -> RGBA32 {
        let scope = unsafe { self.scope.as_ref().unwrap_unchecked() };
        return scope.font_col();
    }

    /// Sets the font col.
    pub fn set_font_col(&self, font_col: RGBA32) {
        let scope = unsafe { self.scope.as_mut().unwrap_unchecked() };
        scope.set_font_col(font_col);
    }

    /// Returns the horizontal alignment for text.
    pub fn text_hor_alignment(&self) -> HorTextAlign {
        let scope = unsafe { self.scope.as_ref().unwrap_unchecked() };
        return scope.text_hor_alignment();
    }

    /// Sets the horizontal alignment for text.
    pub fn set_text_hor_alignment(&self, text_hor_alignment: HorTextAlign) {
        let scope = unsafe { self.scope.as_mut().unwrap_unchecked() };
        scope.set_text_hor_alignment(text_hor_alignment);
    }

    /// Returns the vertical alignment for text.
    pub fn text_ver_alignment(&self) -> VerTextAlign {
        let scope = unsafe { self.scope.as_ref().unwrap_unchecked() };
        return scope.text_ver_alignment();
    }

    /// Sets the vertical alignment for text.
    pub fn set_text_ver_alignment(&self, text_ver_alignment: VerTextAlign) {
        let scope = unsafe { self.scope.as_mut().unwrap_unchecked() };
        scope.set_text_ver_alignment(text_ver_alignment);
    }

    /// Returns the word wrap flag.
    pub fn word_wrap(&self) -> bool {
        let scope = unsafe { self.scope.as_ref().unwrap_unchecked() };
        return scope.word_wrap();
    }

    /// Sets the word wrap flag.
    pub fn set_word_wrap(&self, word_wrap: bool) {
        let scope = unsafe { self.scope.as_mut().unwrap_unchecked() };
        scope.set_word_wrap(word_wrap);
    }

    /// Returns the rich text flag.
    pub fn rich_text(&self) -> bool {
        let scope = unsafe { self.scope.as_ref().unwrap_unchecked() };
        return scope.rich_text();
    }

    /// Sets the rich text flag.
    pub fn set_rich_text(&self, rich_text: bool) {
        let scope = unsafe { self.scope.as_mut().unwrap_unchecked() };
        scope.set_rich_text(rich_text);
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
