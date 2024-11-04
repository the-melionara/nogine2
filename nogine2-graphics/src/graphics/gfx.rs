use nogine2_core::math::{rect::IRect, vector2::ivec2};

use super::{pipeline::RenderStats, texture::rendertex::RenderTexture};

/// Transfer the data from `src` to `dst` retaining integer scaling for pixels.
pub fn integer_scaling_blit(src: &RenderTexture, dst: &RenderTexture, stats: &mut RenderStats) {
    let scaling = integer_scaling::scaling_factor(src.dims(), dst.dims());
    let scaled_src_res = ivec2::from(src.dims() * scaling);

    let offset = (ivec2::from(dst.dims()) - scaled_src_res) / 2;
    dst.combine_ext(src, IRect { start: offset, end: offset + scaled_src_res }, stats);
}

mod integer_scaling {
    use nogine2_core::math::vector2::uvec2;

    pub fn scaling_factor(src_res: uvec2, dst_res: uvec2) -> u32 {
        dst_res.inv_scale(src_res).min_axis().max(1)
    }
}
