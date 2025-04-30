use std::{cmp::Ordering, f32::consts::SQRT_3};

use easy_complex::Complex;
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

/// See (2.23) and (2.24)
fn is_smooth_corner(dadt_1: vec2, dbdt_0: vec2) -> bool {
    const ALPHA: f32 = 1.0;

    return vec2_cross(dadt_1.normalized(), dbdt_0.normalized()).abs() < ALPHA.sin() &&
        dadt_1.dot(dbdt_0) > 0.0;
}

/// See (2.15)
fn vec2_cross(a: vec2, b: vec2) -> f32 {
    a.0 * b.1 - a.1 * b.0
}

/// See (2.43)
fn orthogonality(b_t: vec2, dbdt_t: vec2, p: vec2) -> f32 {
    vec2_cross(dbdt_t.normalized(), (p - b_t).normalized()).abs()
}

/// See (2.41)
fn sdistance(b_t: vec2, dbdt_t: vec2, p: vec2) -> f32 {
    return vec2_cross(dbdt_t, b_t - p).signum() * (b_t - p).magnitude();
}

/// See (2.28) and (2.9), returns `(dist, t)` for convenience
fn dist_to_line(p: vec2, p0: vec2, p1: vec2, pseudo: bool) -> (f32, f32) {
    let mut t = (p - p0).dot(p1 - p0) / (p1 - p0).dot(p1 - p0);
    if !pseudo {
        t = t.clamp(0.0, 1.0);
    }
    return (line(p0, p1, t).dist_to(p), t);
}

/// See (2.38) and (2.9), returns `(dist, t)` for convenience
fn dist_to_bezier2(p: vec2, p0: vec2, p1: vec2, p2: vec2, pseudo: bool) -> (f32, f32) {
    let pv = p - p0;
    let pv1 = p1 - p0;
    let pv2 = p2 - p1 * 2.0 + p0;
    
    let roots = cardano(
        pv2.dot(pv2),
        pv1.dot(pv2) * 3.0,
        pv1.dot(pv1) * 2.0 - pv2.dot(pv),
        pv1.dot(pv),
        !pseudo
    );

    let mut min = f32::INFINITY;
    let mut t_of_min = 0.0;
    for t in roots.as_ref() {
        if !pseudo || (*t >= 0.0 && *t <= 1.0) {
            let new = bezier2(p0, p1, p2, *t).dist_to(p);
            if new < min {
                min = new;
                t_of_min = *t;
            }
        } else {
            let line_t = t.clamp(0.0, 1.0);
            
            let origin = if line_t > 0.5 { p2 } else { p0 };
            let dir = bezier2_dt(p0, p1, p2, line_t);
            
            let new = dist_to_line(p, origin, origin + dir, true).0;
            if new < min {
                min = new;
                t_of_min = *t;
            }
        }
    }
    return (min, t_of_min);
}

/// See (2.40) and (2.9), returns `(dist, t)` for convenience
fn dist_to_bezier3(p: vec2, p0: vec2, p1: vec2, p2: vec2, p3: vec2, pseudo: bool) -> (f32, f32) {
    let pv = p - p0;
    let pv1 = p1 - p0;
    let pv2 = p2 - p1 * 2.0 + p0;
    let pv3 = p3 - p2 * 3.0 + p1 * 3.0 - p0;
    
    let roots = quintic_aproximation(
        pv3.dot(pv3),
        pv2.dot(pv3) * 5.0,
        pv1.dot(pv3) * 4.0 + pv2.dot(pv2) * 6.0,
        pv1.dot(pv2) * 9.0 - pv2.dot(pv),
        pv1.dot(pv1) * 3.0 - pv2.dot(pv) * 2.0,
        pv1.dot(pv),
        !pseudo,
    );

    let mut min = f32::INFINITY;
    let mut t_of_min = 0.0;
    for t in roots.as_ref() {
        if !pseudo || (*t >= 0.0 && *t <= 1.0) {
            let new = bezier3(p0, p1, p2, p3, *t).dist_to(p);
            if new < min {
                min = new;
                t_of_min = *t;
            }
        } else {
            let line_t = t.clamp(0.0, 1.0);
            
            let origin = if line_t > 0.5 { p3 } else { p0 };
            let dir = bezier3_dt(p0, p1, p2, p3, line_t);
            
            let new = dist_to_line(p, origin, origin + dir, true).0;
            if new < min {
                min = new;
                t_of_min = *t;
            }
        }
    }
    return (min, t_of_min);
}

/// Not really only cardano because it also depresses it but whatever
fn cardano(a: f32, b: f32, c: f32, d: f32, bounded: bool) -> Solutions<5> {
    // https://proofwiki.org/wiki/Cardano%27s_Formula

    let q = (3.0 * a * c - b * b) / (9.0 * a * a);
    let r = (9.0 * a * b * c - 27.0 * a * a * d - 2.0 * b * b * b) / (54.0 * a * a * a);

    let s = (r + (q * q * q + r * r).sqrt()).powf(1.0 / 3.0);
    let t = (r - (q * q * q + r * r).sqrt()).powf(1.0 / 3.0);

    let discriminant = q * q * q + r * r;

    let mut res = Solutions::new();
    if bounded {
        res.push(0.0);
        res.push(1.0);
    }

    let x1 = s + t - b / (3.0 * a);
    if !bounded || (x1 > 0.0 && x1 < 1.0) {
        res.push(x1);
    }
            
    match discriminant.partial_cmp(&0.0).unwrap() {
        Ordering::Less | Ordering::Equal => { // all roots are real
            let x2 = Complex::<f32>::new(-(s + t) * 0.5 - b / (3.0 * a), SQRT_3 * 0.5 * (s - t));
            if (!bounded || (x2.real > 0.0 && x2.real < 1.0)) && x2.imag.abs() < f32::EPSILON {
                res.push(x2.real);
            }

            let x3 = Complex::<f32>::new(-(s + t) - 0.5 - b / (3.0 * a), SQRT_3 * 0.5 * (s - t));
            if (!bounded || (x3.real > 0.0 && x3.real < 1.0)) && x3.imag.abs() < f32::EPSILON {
                res.push(x3.real);
            }
        },
        Ordering::Greater => {} // one real root and two complex conjugates
    }
    return res;
}

/// Done with newthon's method. Maybe there's something better, maybe not.
fn quintic_aproximation(
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    e: f32,
    f: f32,
    bounded: bool
) -> Solutions<7> {
    fn g(x: f32, a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) -> f32 {
        let x2 = x * x;
        let x3 = x2 * x;
        let x4 = x3 * x;
        let x5 = x4 * x;

        return a * x5 + b * x4 + c * x3 + d * x2 + e * x + f;
    }

    fn dgdx(x: f32, a: f32, b: f32, c: f32, d: f32, e: f32) -> f32 {
        let x2 = x * x;
        let x3 = x2 * x;
        let x4 = x3 * x;

        return 5.0 * a * x4 + 4.0 * b * x3 + 3.0 * c * x2 + 2.0 * d * x + e;
    }

    let mut solutions = Solutions::new();
    if bounded {
        solutions.push(0.0);
        solutions.push(1.0);
    }

    for i in 0..5 {
        let mut x = i as f32 * 0.2;
        for _ in 0..10 { // 10 iterations, take it or leave it
            let gx = g(x, a, b, c, d, e, f);
            if gx.abs() < f32::EPSILON {
                break;
            }
            x = x - gx / dgdx(x, a, b, c, d, e);
        }

        if !bounded || (x > 0.0 && x < 1.0) {
            solutions.push(x);
        }
    }
    return solutions;
}

fn line(a: vec2, b: vec2, t: f32) -> vec2 {
    return a * (-t + 1.0) +
        b * t;
}

fn line_dt(a: vec2, b: vec2) -> vec2 {
    return b - a;
}

fn bezier2(a: vec2, b: vec2, c: vec2, t: f32) -> vec2 {
    let t2 = t * t;
    return a * (-t2 + 2.0 * t - 1.0) +
        b * (2.0 * t) +
        c * t2;
}

fn bezier2_dt(a: vec2, b: vec2, c: vec2, t: f32) -> vec2 {
    return a * (-2.0 * t + 2.0) +
        b * 2.0 +
        c * (2.0 * t);
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

struct Solutions<const N: usize> {
    inner: [f32; N],
    len: usize,
}

impl<const N: usize> Solutions<N> {
    fn new() -> Self {
        Self { inner: [0.0; N], len: 0 }
    }

    fn push(&mut self, val: f32) {
        assert!(self.len < N);

        self.inner[self.len] = val;
        self.len += 1;
    }

    fn as_ref(&self) -> &[f32] {
        &self.inner[..self.len]
    }
}
