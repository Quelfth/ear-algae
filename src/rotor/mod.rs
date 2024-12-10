use std::ops::Mul;

use crate::traits::*;
use crate::linear::*;

use self::from::FromAngleAxis;
use self::ops::Apl;
use self::ops::BefAft;


pub trait Rot<const N: usize, S: Field>: Sized+Copy+Apl<Vect<N, S>, Output = Vect<N, S>>+Apl<Nrml<N, S>, Output = Nrml<N, S>>+BefAft+FromAngleAxis<S, Self::Axis>+FromFromTo<Nrml<N, S>, Nrml<N, S>> {
    type Bivector;
    type Axis: Mul<S, Output = Self::Bivector>;

    const IDENT: Self;

    /// This is like angle_axis, except that the angle is the magnitude of the input.
    fn from_ang(axis: Self::Bivector) -> Self;

    /// .
    ///
    /// # Safety
    /// Safe when `w*w` + `bi.dot(bi)` = 1.
    /// That is, the created rotor is properly unitized.
    ///
    /// .
    unsafe fn from_w_bi_unchecked(w: S, bi: Self::Bivector) -> Self;

    fn from_ortho(ortho: Ortho<N, S>) -> Self;

    fn angle(self) -> S;
    fn axis(self) -> Option<Self::Axis>;

    fn to_ang(self) -> Self::Bivector;

    fn part(self, t: S) -> Self;
    fn inv(self) -> Self;
}

impl<const N: usize, S: Field, R: Rot<N, S>> FromFromTo<Nrml<N, S>, Option<Nrml<N, S>>> for R {
    fn from_to(from: Nrml<N, S>, to: Option<Nrml<N, S>>) -> Self {
        if let Some(to) = to {
            Self::from_to(from, to)
        } else {
            Self::IDENT
        }
    }
}


pub struct RotBivectorRepr<R>(R);




mod cross_product;


mod rot2;
use from::FromFromTo;
pub use rot2::Rot2;


mod rot3;
pub use rot3::Rot3;
