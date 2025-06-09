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
