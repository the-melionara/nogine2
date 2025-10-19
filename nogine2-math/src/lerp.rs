pub trait Lerp {
    type Factor;
       
    fn lerp(self, other: Self, t: Self::Factor) -> Self;
    fn clamped_lerp(self, other: Self, t: Self::Factor) -> Self;
}

pub trait CompLerp {
    type VFactor;

    fn clerp(self, other: Self, t: Self::VFactor) -> Self;
    fn clamped_clerp(self, other: Self, t: Self::VFactor) -> Self;
}

impl Lerp for f32 {
    type Factor = f32;

    fn lerp(self, other: Self, t: Self::Factor) -> Self {
        return self * (1.0 - t) + other * t;
    }

    fn clamped_lerp(self, other: Self, t: Self::Factor) -> Self {
        self.lerp(other, t.clamp(0.0, 1.0))
    }
}
