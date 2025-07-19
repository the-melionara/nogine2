use super::engine::LineData;

pub enum HorTextAlign {
    Left, Center, Right, //Expand, Justified,
}

impl HorTextAlign {
    pub(crate) fn dx0_and_spaces(
        &self,
        extents_width: f32,
        space_width: f32,
        line_data: &LineData,
    ) -> (f32, f32) {
        match (self, line_data.word_wrapped) {
            (HorTextAlign::Left, _) /*| (HorTextAlign::Justified, false)*/ => (0.0, space_width),
            (HorTextAlign::Center, _) => ((extents_width - line_data.min_width) * 0.5, space_width),
            (HorTextAlign::Right, _) => (extents_width - line_data.min_width, space_width),
            // (HorTextAlign::Expand, _) | (HorTextAlign::Justified, true) => {
            //     let final_wordless_width = extents_width - line_data.spaceless_width;
            //     (0.0, final_wordless_width / line_data.space_count as f32)
            // }
        }
    }
}


pub enum VerTextAlign {
    Top, Center, Bottom, //Expand,
}

impl VerTextAlign {
    pub(crate) fn dy0_and_spaces(
        &self,
        extents_height: f32,
        line_height: f32,
        line_count: usize
    ) -> (f32, f32) {
        match self {
            VerTextAlign::Top => (0.0, line_height),
            VerTextAlign::Center => (
                (extents_height - line_height * line_count as f32) * 0.5,
                line_height,
            ),
            VerTextAlign::Bottom => (
                extents_height - line_height * line_count as f32,
                line_height,
            ),
            // VerTextAlign::Expand => {
            //     (0.0, extents_height / line_count as f32)
            // },
        }
    }
}
