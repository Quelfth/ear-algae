use std::fmt::Display;

use restricted::Restricted;
use serde::{Deserialize, Serialize};

use self::ops::*;

use super::*;

use crate::{smath, vect};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(
    from = "Rot2AngRepr<S>", 
    into = "Rot2AngRepr<S>",
)]
pub struct Rot2<S: Field> (S, Vect<1, S>);

impl<S: Field> Rot2<S> {
    fn renormalize(self) -> Self {
        if let Some(normal) = vect!(self.0, self.1[0]).normal() {
            Self(normal[0], vect!(normal[1]))
        } else {
            Self::IDENT
        }
    }
}

impl<S: Field> FromAngleAxis<S, Nrml<1, S>> for Rot2<S> {
    fn angle_axis(angle: S, axis: Nrml<1, S>) -> Self {
        let (sin, cos) = smath!{ (angle/2).sin_cos };
        Self(cos, axis * sin)
    }
}


impl<S: Field> FromAngleAxis<S, Option<Nrml<1, S>>> for Rot2<S> {
    fn angle_axis(angle: S, axis: Option<Nrml<1, S>>) -> Self {
        match axis {
            Some(axis) => Self::angle_axis(angle, axis),
            None => Self::IDENT
        }
    }
}

impl<S: Field> FromFromTo<Nrml<2, S>, Nrml<2, S>> for Rot2<S> {
    fn from_to(from: Nrml<2, S>, to: Nrml<2, S>) -> Self {
        let dot = to.dot(from);
        let cross = from.cross(to);
        if cross == Vect::ZERO {
            if dot > S::ZERO {
                return Self::IDENT;
            } else {
                return Self(S::ZERO, Vect::axis(0, S::ONE));
            }
        }
        Self(dot, cross).part(S::HALF)
    }
}


impl<S: Field> Rot2<S> {
    pub fn angle(angle: S) -> Self {
        Self::angle_axis(angle, Nrml::axis(0))
    }

    pub fn signed_angle(self) -> S {
        self.angle().mul(self.axis().relax_or_zero()[0]).add(S::PI).rem_euclid(S::PI.add(S::PI)).sub(S::PI)
    }

    pub fn lift<const N: usize, R: Rot<N, S>>(self, axis: R::Axis) -> R {
        let w = self.0;
        let bi = axis * self.1[0];
        unsafe {R::from_w_bi_unchecked(w, bi)}
    }
}




impl<S: Field> Rot<2, S> for Rot2<S> {
    type Bivector = Vect<1, S>;
    type Axis = Nrml<1, S>;

    const IDENT: Self = Self(S::ONE, Vect::ZERO);

    fn from_ang(ang: Self::Bivector) -> Self {
        if let Some((angle, axis)) = ang.magn_normal() {
            Self::angle_axis(angle, axis)
        } else {
            Self::IDENT
        }
    }

    

    
    unsafe fn from_w_bi_unchecked(w: S, bi: Self::Bivector) -> Self {
        Self(w, bi)
    }

    fn from_ortho(ortho: Ortho<2, S>) -> Self {
        Self::from_to(Nrml::axis(1), ortho[1])
    }
    

    fn angle(self) -> S {
        let w = self.0.clamp(S::ONE.neg(), S::ONE);
        smath!{ 2 * (w.acos) }
    }

    fn axis(self) -> Option<Self::Axis> {
        self.1.normal()
    }

    fn to_ang(self) -> Self::Bivector {
        self.axis().relax_or_zero() * self.angle()
    }

    fn part(self, t: S) -> Self {
        Self::angle_axis(self.angle().mul(t), self.1.normal())
    }

    fn inv(self) -> Self {
        Self(self.0, -self.1)
    }
}


impl<S: Field> Apl<Vect<2, S>> for Rot2<S> {
    type Output = Vect<2, S>;
    fn apl(self, vect: Vect<2, S>) -> Vect<2, S> {
        (vect * self.0.pow(2).sub(self.1.dot(self.1))) + (vect!(vect[1].neg(), vect[0]) * self.1[0] * self.0) * S::TWO
    }
}

impl<S: Field> Apl<Nrml<2, S>> for Rot2<S> {
    type Output = Nrml<2, S>;
    fn apl(self, vect: Nrml<2, S>) -> Nrml<2, S> {
        self.apl(vect.relax()).normal().unwrap()
    }
}

impl<S: Field> BefAft for Rot2<S> {
    fn aft(self, other: Self) -> Self {
        Self (
            self.0.mul(other.0).sub(self.1.dot(other.1)),
            other.1 * self.0 + self.1 * other.0
        ).renormalize()
    }
}

impl<S: Field+Display> Display for Rot2<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}π rad", self.angle().div(S::PI).mul(self.axis().relax_or_zero()[0])))

    }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
#[serde(transparent)]
struct Rot2AngRepr<S: Field>(<Rot2<S> as Rot<2, S>>::Bivector);


impl<S: Field> From<Rot2<S>> for Rot2AngRepr<S> {
    fn from(value: Rot2<S>) -> Self {
        Rot2AngRepr(value.to_ang())
    }
}

impl<S: Field> From<Rot2AngRepr<S>> for Rot2<S> {
    fn from(value: Rot2AngRepr<S>) -> Self {
        Self::from_ang(value.0)
    }
}