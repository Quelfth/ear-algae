use culit::culit;

use super::*;

use crate::{
    op_wrapper::{Sc, scs},
    ops::{Cross, Dot},
};

impl RotDim<2> for () {
    type Inner<S: Field> = RotInner2<S>;
}

#[derive(Copy, Clone, Debug)]
pub struct RotInner2<S: Field>(S, Vect<1, S>);

impl<S: Field> RotInner2<S> {
    fn renormalize(self) -> Self {
        if let Some(normal) = Vect([self.0, self.1[0]]).normal() {
            Self(normal[0], Vect([normal[1]]))
        } else {
            Self::IDENT
        }
    }
}

impl<S: Field> RotInner2<S> {
    pub fn angle(angle: S) -> Self {
        Self::angle_axis(angle, Nrml::axis(0))
    }

    pub fn signed_angle(self) -> S {
        self.angle()
            .mul(self.1.normal_or_zero()[0])
            .add(S::PI)
            .rem_euclid(S::PI.add(S::PI))
            .sub(S::PI)
    }

    pub fn lift<const N: usize, R: RotInner<N, S>>(self, axis: R::Axis) -> R {
        let w = self.0;
        let bi = axis * self.1[0];
        unsafe { R::from_w_bi_unchecked(w, bi) }
    }
}

impl<S: Field> RotInner<2, S> for RotInner2<S> {
    type Bivector = Vect<1, S>;
    type Axis = Nrml<1, S>;

    const IDENT: Self = Self(S::ONE, Vect::ZERO);

    #[culit]
    fn angle_axis(angle: S, axis: Self::Axis) -> Self {
        let (sin, cos) = (Sc(angle) / 2Sc).sin_cos();
        Self(cos.0, axis * sin.0)
    }

    #[culit]
    fn from_to(from: Nrml<2, S>, to: Nrml<2, S>) -> Self {
        let dot = to.dot(from);
        let cross = from.cross(to);
        if cross == Vect::ZERO {
            if dot > 0S {
                return Self::IDENT;
            } else {
                return Self(0S, Vect::axis(0, 1S));
            }
        }

        let sqrt = (Sc(dot) + 1Sc).max(0Sc).sqrt();
        Self((sqrt / Sc::SQRT_2).0, cross / (sqrt * Sc::SQRT_2).0)
    }

    fn from_torq(ang: Self::Bivector) -> Self {
        if let Some((angle, axis)) = ang.magn_normal() {
            Self::angle_axis(angle, axis)
        } else {
            Self::IDENT
        }
    }

    unsafe fn from_w_bi_unchecked(w: S, bi: Self::Bivector) -> Self {
        Self(w, bi)
    }

    #[culit]
    fn angle(self) -> S {
        let w = Sc(self.0).clamp(-1Sc, 1Sc);
        (2Sc * w.acos()).0
    }

    fn axis(self) -> Option<Self::Axis> {
        self.1.normal()
    }

    fn axis_or_zero(self) -> Self::Bivector {
        self.1.normal_or_zero()
    }

    fn w(self) -> S {
        self.0
    }

    fn bi(self) -> Self::Bivector {
        self.1
    }

    fn to_torq(self) -> Self::Bivector {
        self.1.normal_or_zero() * self.angle()
    }

    fn part(self, t: S) -> Self {
        if let Some(normal) = self.1.normal() {
            Self::angle_axis(self.angle().mul(t), normal)
        } else {
            Self::IDENT
        }
    }

    fn inv(self) -> Self {
        Self(self.0, -self.1)
    }

    fn aft(self, other: Self) -> Self {
        Self(
            self.0.mul(other.0).sub(self.1.dot(other.1)),
            other.1 * self.0 + self.1 * other.0,
        )
        .renormalize()
    }

    #[culit]
    fn apl(self, vect: Vect<2, S>) -> Vect<2, S> {
        (vect * self.0.pow(2).sub(self.1.dot(self.1)))
            + (Vect([vect[1].neg(), vect[0]]) * self.1[0] * self.0) * 2S
    }

    fn normalize_bivector(vector: Self::Bivector) -> Option<Self::Axis> {
        vector.normal()
    }

    fn mat(self) -> Mat<2, 2, S> {
        let Self(cos, Vect([sin])) = self;
        scs!(cos, sin);
        Mat::from_scs([[cos, -sin], [sin, cos]])
    }
}
