use nogine2_math::{lerp::Lerp, vector2::{uvec2, vec2}};
use ttf_parser::{Face, OutlineBuilder};

pub struct MSDFGenResult {
    pub pixels: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

pub struct MSDFGenConfig {
    pub pixels_per_em: u32,
    pub field_range: f32,
}

pub struct MSDFGenerator<'a> {
    face: Face<'a>,
    cfg: MSDFGenConfig,
}

impl<'a> MSDFGenerator<'a> {
    const TEST_CHAR: char = 'A';
    
    pub fn new(data: &'a [u8], cfg: MSDFGenConfig) -> Self {
        let face = ttf_parser::Face::parse(data, 0).unwrap();
        let index = face.glyph_index('E').unwrap();
        let glyf_table = face.tables().glyf.unwrap();

        // let mut builder = ContourIterator::new(face.tables().head.units_per_em as f32, (0.0, 0.0));
        // glyf_table.outline(index, &mut builder);
        // let lines = builder.lines();

        return Self { face, cfg };
    }

    pub fn run(self) -> Option<MSDFGenResult> {
        let target_size = self.target_size()?;
        let mut res = vec![0; target_size.0 as usize * target_size.1 as usize];

        for i in 0..target_size.0 {
            for j in 0..target_size.1 {
                let pos = self.tex2em_space(uvec2(i, j))?;
                self.gen_pixel(pos, (i + j * target_size.0) as usize, &mut res);
            }
        }

        return Some(MSDFGenResult { pixels: res, width: target_size.0, height: target_size.1 });
    }

    fn gen_pixel(&self, pos: vec2, index: usize, res: &mut [u8]) {
        res[index] = (pos.0.clamp(0.0, 1.0) * 255.0) as u8; // Very simple test!!
    }

    fn target_size(&self) -> Option<uvec2> {
        let bb = self.face.glyph_bounding_box(self.face.glyph_index(Self::TEST_CHAR)?)?;
        if bb.width() <= 0 || bb.height() <= 0 {
            return None;
        }
        
        let pixels_per_em = self.cfg.pixels_per_em as u64;
        let units_per_em = self.face.units_per_em() as u64;
        
        let width = (bb.width() as u64 * pixels_per_em)
            .div_ceil(self.face.units_per_em() as u64) as u32;

        let height = (bb.height() as u64 * pixels_per_em)
            .div_ceil(units_per_em) as u32;

        return Some(uvec2(width, height));
    }

    fn tex2em_space(&self, pos: uvec2) -> Option<vec2> {
        // I HATE MATHS AND I HATE FONTS
        let bb = self.face.glyph_bounding_box(self.face.glyph_index(Self::TEST_CHAR)?)?; // u
        if bb.width() <= 0 || bb.height() <= 0 {
            return None;
        }

        let units_per_em = self.face.units_per_em() as f32; // u/em
        let pixels_per_em = self.cfg.pixels_per_em as f32; // px/em

        let bb_offset_em = vec2(bb.x_min as f32, bb.y_min as f32) / units_per_em; // em
        return Some(vec2::from(pos) / pixels_per_em + bb_offset_em); // em
    }
}

struct ContourIterator {
    origin: vec2,
    units_per_em: f32,
    point: vec2,

    min_dist: f32,
}

impl ContourIterator {
    fn new(units_per_em: f32, point: vec2) -> Self {
        Self { origin: vec2::ZERO, units_per_em, point, min_dist: f32::INFINITY }
    }

    fn min_dist(self) -> f32 {
        self.min_dist
    }
}

impl OutlineBuilder for ContourIterator {
    fn move_to(&mut self, x: f32, y: f32) {
        println!("MOVE_TO: {x}, {y}");
        self.origin = vec2(x / self.units_per_em, y / self.units_per_em);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        println!("LINE_TO: {x}, {y}");
        self.origin = vec2(x / self.units_per_em, y / self.units_per_em);
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        println!("QUAD_TO: {x1}, {y1}, {x}, {y}");
        self.origin = vec2(x / self.units_per_em, y / self.units_per_em);
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        println!("CURVE_TO: {x1}, {y1}, {x2}, {y2}, {x}, {y}");
        self.origin = vec2(x / self.units_per_em, y / self.units_per_em);
    }

    fn close(&mut self) {
        println!("END");
    }
}

// fn cubic_bezier(a: vec2, b: vec2, c: vec2, d: vec2, t: f32) -> vec2 {
//     let ab = a.lerp(b, t);
//     let bc = b.lerp(c, t);
//     let cd = c.lerp(d, t);

//     let ac = ab.lerp(bc, t);
//     let bd = bc.lerp(cd, t);

//     return ac.lerp(bd, t);
// }

/// As defined by the paper
fn vec2_cross(a: vec2, b: vec2) -> f32 {
    a.0 * b.1 - a.1 * b.0
}

fn bezier2(a: vec2, b: vec2, c: vec2, t: f32) -> vec2 {
    let t2 = t * t;
    return (c - a) * t2 + (b - a) * 2.0 * t + a;
}

fn bezier2_dt(a: vec2, b: vec2, c: vec2, t: f32) -> vec2 {
    let ab = a.lerp(b, t);
    let bc = b.lerp(c, t);
    return ab.lerp(bc, t);
}


fn bezier3(a: vec2, b: vec2, c: vec2, d: vec2, t: f32) -> vec2 {
    // Ripped straight from Freya Holmér's video on beziers (bless her heart)
    let t3 = t * t * t;
    let t2 = t * t;
    
    return a * (-t3 + 3.0 * t2 - 3.0 * t + 1.0) +
        b * (3.0 * t3 - 6.0 * t2 + 3.0 * t) +
        c * (-3.0 * t3 + 3.0 * t2) +
        d * t3;
}

fn bezier3_dt(a: vec2, b: vec2, c: vec2, d: vec2, t: f32) -> vec2 {
    // Ripped straight from Freya Holmér's video on beziers (bless her heart)
    let t2 = t * t;
    
    return a * (-3.0 * t2 + 6.0 * t - 3.0) +
        b * (9.0 * t2 - 12.0 * t + 3.0) +
        c * (-9.0 * t2 + 6.0 * t) +
        d * (3.0 * t2);
}
