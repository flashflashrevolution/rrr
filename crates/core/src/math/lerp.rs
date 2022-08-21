use num_traits::Float;
use std::ops::{Add, Mul, Sub};

pub trait Lerp<F> {
    fn lerp(self, other: Self, t: F) -> Self;
    fn inv_lerp(self, other: Self, v: Self) -> F;
}

impl<T, F> Lerp<F> for T
where
    T: Add<Output = T> + Mul<F, Output = T> + Sub<Output = F>,
    F: Float,
    Self: Copy,
{
    fn lerp(self, other: T, t: F) -> T {
        self * (F::one() - t) + other * t
    }

    fn inv_lerp(self, other: T, v: T) -> F {
        (v - self) / (other - self)
    }
}
