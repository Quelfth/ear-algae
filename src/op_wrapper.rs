use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::traits::{Field, Ring};

macro_rules! scs {
    ($($s:tt),* $(,)?) => {$(
        let $s = $crate::op_wrapper::Sc($s);
    )*}
}
pub(crate) use scs;

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Sc<S>(pub S);

impl<S: Ring> Add for Sc<S> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.add(rhs.0))
    }
}

impl<S: Ring> Sub for Sc<S> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0.sub(rhs.0))
    }
}

impl<S: Ring> Mul for Sc<S> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0.mul(rhs.0))
    }
}

impl<S: Ring> Div for Sc<S> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0.div(rhs.0))
    }
}

impl<S: Ring> Neg for Sc<S> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(self.0.neg())
    }
}

impl<S: Ring> Sc<S> {
    pub fn pow(self, other: u32) -> Self {
        Self(self.0.pow(other))
    }

    pub fn max(self, other: Self) -> Self {
        Self(self.0.max(other.0))
    }
    pub fn min(self, other: Self) -> Self {
        Self(self.0.min(other.0))
    }

    pub fn clamp(self, min: Self, max: Self) -> Self {
        Self(self.0.clamp(min.0, max.0))
    }
}

impl<S: Field> Sc<S> {
    pub const SQRT_2: Self = Self(S::SQRT_2);
    pub const PI: Self = Self(S::PI);

    pub fn sqrt(self) -> Self {
        Self(self.0.sqrt())
    }

    pub fn exp(self) -> Self {
        Self(self.0.exp())
    }

    pub fn sin(self) -> Self {
        Self(self.0.sin())
    }
    pub fn cos(self) -> Self {
        Self(self.0.cos())
    }
    pub fn tan(self) -> Self {
        Self(self.0.tan())
    }

    pub fn sin_cos(self) -> (Self, Self) {
        let (sin, cos) = self.0.sin_cos();
        (Self(sin), Self(cos))
    }

    pub fn ln(self) -> Self {
        Self(self.0.ln())
    }

    pub fn asin(self) -> Self {
        Self(self.0.asin())
    }
    pub fn acos(self) -> Self {
        Self(self.0.acos())
    }
    pub fn atan(self) -> Self {
        Self(self.0.atan())
    }

    pub fn atan2(y: Self, x: Self) -> Self {
        Self(S::atan2(y.0, x.0))
    }
}
