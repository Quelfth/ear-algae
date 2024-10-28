use std::fmt::{self, Display};

use restricted::Restricted;

use crate::{smath, smath_mat, vect};

use super::*;
use super::ops::*;

#[derive(Copy, Clone)]
pub struct Rot3<S: Field> (S, Vect<3, S>);

impl<S: Field> FromAngleAxis<S, Nrml<3, S>> for Rot3<S> {
    fn angle_axis(angle: S, axis: Nrml<3, S>) -> Self {
        let (sin, cos) = smath!{ (angle/2).sin_cos };
        Self(cos, axis * sin)
    }
}

impl<S: Field> FromAngleAxis<S, Option<Nrml<3, S>>> for Rot3<S> {
    fn angle_axis(angle: S, axis: Option<Nrml<3, S>>) -> Self {
        match axis {
            Some(axis) => Self::angle_axis(angle, axis),
            None => Self::IDENT
        }
    }
}

impl<S: Field> FromFromTo<Nrml<3, S>, Nrml<3, S>> for Rot3<S> {
    fn from_to(from: Nrml<3, S>, to: Nrml<3, S>) -> Self {
        let dot = to.dot(from);
        let cross = from.cross(to);
        if cross == Vect::ZERO { //This all handles the edge case where from == -to
            if dot > S::ZERO {
                return Self::IDENT;
            }
            
            if let Some(axis) = from.cross(Nrml::axis(0)).normal() {
                return Self(S::ZERO, axis.relax());
            } else {
                return Self(S::ZERO, Vect::axis(1, S::ONE));
            }
        }
        Self(dot, cross).part(S::HALF)
    }
}





impl<S: Field> Rot<3, S> for Rot3<S> { 

    type Bivector = Vect<3, S>;
    type Axis = Nrml<3, S>;

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
    
    fn from_ortho(ortho: Ortho<3, S>) -> Self {
        let z = Self::from_to(Nrml::axis(2), ortho[2]);
        let y = Self::from_to(z.apl(Nrml::axis(1)), ortho[1]);
        z.bef(y)
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

impl<S: Field> Apl<Vect<3, S>> for Rot3<S> {
    type Output = Vect<3, S>;
    fn apl(self, vect: Vect<3, S>) -> Vect<3, S> {
        (vect * self.0.pow(2).sub(self.1.dot(self.1))) + (self.1 * self.1.dot(vect) + self.1.cross(vect) * self.0) * S::TWO
    }
}

impl<S: Field> Apl<Nrml<3, S>> for Rot3<S> {
    type Output = Nrml<3, S>;
    fn apl(self, vect: Nrml<3, S>) -> Nrml<3, S> {
        self.apl(vect.relax()).normal().unwrap()
    }
}

impl<S: Field> BefAft for Rot3<S> {
    fn bef(self, other: Self) -> Self {
        other.aft(self)
    }

    fn aft(self, other: Self) -> Self {
        Self (
            self.0.mul(other.0).sub(self.1.dot(other.1)),
            self.1.cross(other.1) + other.1 * self.0 + self.1 * other.0
        )
    }
}

impl<S: Field> Rot3<S> {
    pub fn mat(self) -> Mat<3,3,S> {
        let [x,y,z] = self.1.as_array();
        let w = self.0;

        Mat::IDENT + (smath_mat![
            (-((y^2) + (z^2))), ((x*y) - (z*w)), ((z*x) + (y*w));
            ((x*y) + (z*w)), (-((z^2) + (x^2))), ((y*z) - (x*w));
            ((z*x) - (y*w)), ((y*z) + (x*w)), (-((x^2) + (y^2)))
        ] 
        * S::TWO)
    }
}

impl<S: Field> Rot3<S> {
    pub fn pitch(angle: S) -> Self {
        let (sin, cos) = smath!{(angle/2).sin_cos};
        Self(cos, vect!(sin, S::ZERO, S::ZERO))
    }

    pub fn yaw(angle: S) -> Self {
        let (sin, cos) = smath!{(angle/2).sin_cos};
        Self(cos, vect!(S::ZERO, sin, S::ZERO))
    }

    pub fn roll(angle: S) -> Self {
        let (sin, cos) = smath!{(angle/2).sin_cos};
        Self(cos, vect!(S::ZERO, S::ZERO, sin))
    }
}

impl<S: Field+Display> Display for Rot3<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}π rad {}", self.angle().div(S::PI), self.axis().relax_or_zero()))

    }
}

impl<S: Field+fmt::Debug> fmt::Debug for Rot3<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Rot").field(&self.angle()).field(&self.axis().relax_or_zero()).finish()
    }
}

