use std::cmp::Ordering;

mod impls;
pub mod ops;
pub mod convert;
pub mod from;
pub mod restricted;

pub trait Ring: Copy+Sized+PartialEq+PartialOrd {
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;
    fn add(self, other: Self) -> Self;
    fn add_assign(&mut self, other: Self);
    fn mul(self, other: Self) -> Self;
    fn mul_assign(&mut self, other: Self);
    fn sub(self, other: Self) -> Self;
    fn sub_assign(&mut self, other: Self);
    fn div(self, other: Self) -> Self;
    fn div_assign(&mut self, other: Self);
    fn pow(self, other: u32) -> Self;
    fn neg(self) -> Self;
    fn abs(self) -> Self;
    fn sign(self) -> Self;
    fn cmp(&self, other: &Self) -> Ordering;
    fn is_zero(self) -> bool;
    fn is_nan(self) -> bool;
    fn is_finite(self) -> bool;
    fn rem_euclid(self, other: Self) -> Self;

    fn clamp(self, min: Self, max: Self) -> Self {
        if self.cmp(&min).is_lt() {
            min
        }
        else if self.cmp(&max).is_gt() {
            max
        }
        else {
            self
        }
    }
}

pub trait Field : Ring {
    const HALF: Self;
    const PI: Self;
    const INFINITY: Self;

    fn sqrt(self) -> Self;

    fn exp(self) -> Self;

    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;

    fn sin_cos(self) -> (Self, Self);

    fn ln(self) -> Self;

    fn asin(self) -> Self;
    fn acos(self) -> Self;
    fn atan(self) -> Self;

    fn atan2(y: Self, x: Self) -> Self;
}