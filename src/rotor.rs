use crate::{
    Mat, Nrml, Vect,
    op_wrapper::{Sc, scs},
    ops::{Apl, BefAft},
    traits::Field,
};
use std::ops::Mul;

use culit::culit;
use maybe_trait::Maybe;

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

mod cross_product;
mod rot2;
mod rot3;

pub trait RotInner<const N: usize, S: Field>: Sized + Copy {
    type Bivector;
    type Axis: Mul<S, Output = Self::Bivector>;

    const IDENT: Self;

    fn angle_axis(angle: S, axis: Self::Axis) -> Self;
    fn from_to(from: Nrml<N, S>, to: Nrml<N, S>) -> Self;

    /// This is like angle_axis, except that the angle is the magnitude of the input.
    fn from_torq(axis: Self::Bivector) -> Self;

    /// .
    ///
    /// # Safety
    /// Safe when `w*w` + `bi.dot(bi)` = 1.
    /// That is, the created rotor is properly unitized.
    ///
    /// .
    unsafe fn from_w_bi_unchecked(w: S, bi: Self::Bivector) -> Self;

    fn angle(self) -> S;
    fn axis(self) -> Option<Self::Axis>;

    fn axis_or_zero(self) -> Self::Bivector;

    fn w(self) -> S;
    fn bi(self) -> Self::Bivector;

    fn to_torq(self) -> Self::Bivector;

    fn part(self, t: S) -> Self;
    fn inv(self) -> Self;

    fn aft(self, other: Self) -> Self;

    fn apl(self, vect: Vect<N, S>) -> Vect<N, S>;

    fn normalize_bivector(vector: Self::Bivector) -> Option<Self::Axis>;

    fn mat(self) -> Mat<N, N, S>;
}

pub struct RotBivectorRepr<R>(R);

pub trait RotDim<const N: usize> {
    type Inner<S: Field>: RotInner<N, S>;
}

#[derive(Copy, Clone)]
pub struct Rot<const N: usize, S: Field>(<() as RotDim<N>>::Inner<S>)
where
    (): RotDim<N>;
pub(crate) type Axis<const N: usize, S> = <<() as RotDim<N>>::Inner<S> as RotInner<N, S>>::Axis;
pub(crate) type Bivector<const N: usize, S> =
    <<() as RotDim<N>>::Inner<S> as RotInner<N, S>>::Bivector;

impl<const N: usize, S: Field> Default for Rot<N, S>
where
    (): RotDim<N>,
{
    fn default() -> Self {
        Self(RotInner::IDENT)
    }
}

impl<const N: usize, S: Field> Rot<N, S>
where
    (): RotDim<N>,
{
    pub const IDENT: Self = Self(RotInner::IDENT);

    pub fn angle_axis(angle: S, axis: impl Maybe<Axis<N, S>>) -> Self {
        if let Some(axis) = axis.maybe() {
            Self(RotInner::angle_axis(angle, axis))
        } else {
            Self::IDENT
        }
    }

    pub fn from_to(from: impl Maybe<Nrml<N, S>>, to: impl Maybe<Nrml<N, S>>) -> Self {
        if let Some(from) = from.maybe()
            && let Some(to) = to.maybe()
        {
            Self(RotInner::from_to(from, to))
        } else {
            Self::IDENT
        }
    }

    /// # Safety
    /// Safe when `w*w + bi.dot(bi) =~ 1.`
    /// That is, the created rotor is properly unitized.
    pub unsafe fn from_w_bi_unchecked(w: S, bi: Bivector<N, S>) -> Self {
        Self(unsafe { RotInner::from_w_bi_unchecked(w, bi) })
    }

    pub fn from_torq(torq: Bivector<N, S>) -> Self {
        Self(RotInner::from_torq(torq))
    }

    pub fn angle(self) -> S {
        self.0.angle()
    }
    pub fn axis(self) -> Option<Axis<N, S>> {
        self.0.axis()
    }

    pub fn axis_or_zero(self) -> Bivector<N, S> {
        self.0.axis_or_zero()
    }

    pub fn w(self) -> S {
        self.0.w()
    }

    pub fn bi(self) -> Bivector<N, S> {
        self.0.bi()
    }

    pub fn to_torq(self) -> Bivector<N, S> {
        self.0.to_torq()
    }

    pub fn part(self, t: S) -> Self {
        Self(self.0.part(t))
    }
    pub fn inv(self) -> Self {
        Self(self.0.inv())
    }

    pub fn mat(self) -> Mat<N, N, S> {
        self.0.mat()
    }
}

impl<const N: usize, S: Field> BefAft for Rot<N, S>
where
    (): RotDim<N>,
{
    fn aft(self, other: Self) -> Self {
        Self(self.0.aft(other.0))
    }
}

impl<const N: usize, S: Field> Apl<Vect<N, S>> for Rot<N, S>
where
    (): RotDim<N>,
{
    type Output = Vect<N, S>;

    fn apl(self, other: Vect<N, S>) -> Self::Output {
        RotInner::apl(self.0, other)
    }
}

impl<const N: usize, S: Field> Apl<Nrml<N, S>> for Rot<N, S>
where
    (): RotDim<N>,
{
    type Output = Nrml<N, S>;

    fn apl(self, other: Nrml<N, S>) -> Self::Output {
        RotInner::apl(self.0, other.into()).normal().unwrap()
    }
}

impl<S: Field> Rot<2, S> {
    pub fn angle2(angle: S) -> Self {
        Self::angle_axis(angle, Nrml::axis(0))
    }

    pub fn signed_angle(self) -> S {
        self.angle()
            .mul(self.bi().normal_or_zero()[0])
            .add(S::PI)
            .rem_euclid(S::PI.add(S::PI))
            .sub(S::PI)
    }

    pub fn lift<const N: usize, R: RotInner<N, S>>(self, axis: R::Axis) -> R {
        let w = self.w();
        let bi = axis * self.bi()[0];
        unsafe { R::from_w_bi_unchecked(w, bi) }
    }
}

impl<S: Field> Rot<3, S> {
    #[culit]
    pub fn pitch(angle: S) -> Self {
        let (sin, cos) = (Sc(angle) / 2Sc).sin_cos();
        unsafe { Self::from_w_bi_unchecked(cos.0, Vect::axis(0, sin.0)) }
    }

    #[culit]
    pub fn yaw(angle: S) -> Self {
        let (sin, cos) = (Sc(angle) / 2Sc).sin_cos();
        unsafe { Self::from_w_bi_unchecked(cos.0, Vect::axis(1, sin.0)) }
    }

    #[culit]
    pub fn roll(angle: S) -> Self {
        let (sin, cos) = (Sc(angle) / 2Sc).sin_cos();
        unsafe { Self::from_w_bi_unchecked(cos.0, Vect::axis(2, sin.0)) }
    }

    #[culit]
    pub fn euler_angles(self) -> (S, S, S) {
        let w = self.w();
        let Vect([x, y, z]) = self.bi();
        scs!(w, x, y, z);

        (
            Sc::atan2(2Sc * (w * y + z * x), 1Sc - 2Sc * (y.pow(2) + x.pow(2))).0,
            (2Sc * (w * x - (y - z))).clamp(-1Sc, 1Sc).asin().0,
            Sc::atan2(2Sc * (w * z + x * y), 1Sc - 2Sc * (z.pow(2) + x.pow(2))).0,
        )
    }
}

#[cfg(feature = "serde")]
impl<const N: usize, S: Field> Serialize for Rot<N, S>
where
    (): RotDim<N>,
    Bivector<N, S>: Serialize,
{
    fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error> {
        self.to_torq().serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, const N: usize, S: Field> Deserialize<'de> for Rot<N, S>
where
    (): RotDim<N>,
    Bivector<N, S>: Deserialize<'de>,
{
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::from_torq(Bivector::<N, S>::deserialize(
            deserializer,
        )?))
    }
}
