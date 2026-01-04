use culit::culit;

use super::*;

use crate::{
    op_wrapper::{Sc, scs},
    ops::{Cross as _, Dot as _},
};

impl RotDim<3> for () {
    type Inner<S: Field> = RotInner3<S>;
}

#[derive(Copy, Clone)]
pub struct RotInner3<S: Field>(S, Vect<3, S>);

impl<S: Field> RotInner3<S> {
    fn renormalize(self) -> Self {
        if let Some(normal) = Vect([self.0, self.1[0], self.1[1], self.1[2]]).normal() {
            Self(normal[0], Vect([normal[1], normal[2], normal[3]]))
        } else {
            Self::IDENT
        }
    }
}

impl<S: Field> RotInner<3, S> for RotInner3<S> {
    type Bivector = Vect<3, S>;
    type Axis = Nrml<3, S>;

    const IDENT: Self = Self(S::ONE, Vect::ZERO);

    #[culit]
    fn angle_axis(angle: S, axis: Nrml<3, S>) -> Self {
        let (sin, cos) = (Sc(angle) / 2Sc).sin_cos();
        Self(cos.0, axis * sin.0)
    }

    fn from_to(from: Nrml<3, S>, to: Nrml<3, S>) -> Self {
        let dot = to.dot(from);
        let cross = from.cross(to);
        if cross == Vect::ZERO {
            //This all handles the edge case where from == -to
            if dot > S::ZERO {
                return Self::IDENT;
            }

            if let Some(axis) = from.cross(Nrml::axis(0)).normal() {
                return Self(S::ZERO, axis.into());
            } else {
                return Self(S::ZERO, Vect::axis(1, S::ONE));
            }
        }

        let sqrt = dot.add(S::ONE).max(S::ZERO).sqrt();
        Self(sqrt.div(S::SQRT_2), cross / sqrt.mul(S::SQRT_2))
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
        (2Sc * (w.acos())).0
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
            self.1.cross(other.1) + other.1 * self.0 + self.1 * other.0,
        )
        .renormalize()
    }

    fn apl(self, vect: Vect<3, S>) -> Vect<3, S> {
        (vect * self.0.pow(2).sub(self.1.dot(self.1)))
            + (self.1 * self.1.dot(vect) + self.1.cross(vect) * self.0) * S::TWO
    }

    fn normalize_bivector(vector: Self::Bivector) -> Option<Self::Axis> {
        vector.normal()
    }

    #[culit]
    fn mat(self) -> Mat<3, 3, S> {
        let Self(w, Vect([x, y, z])) = self;
        scs!(w, x, y, z);

        Mat::IDENT
            + Mat::from_scs([
                [-(y.pow(2) + z.pow(2)), x * y - z * w, z * x + y * w],
                [x * y + z * w, -(z.pow(2) + x.pow(2)), y * z - x * w],
                [z * x - y * w, y * z + x * w, -(x.pow(2) + y.pow(2))],
            ]) * 2S
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_to() {
        let v1: Nrml<3, f32> = Vect([-2.34, 5.8, -0.8]).normal().unwrap();
        let v2 = Vect([-8.2, 1.1, 4.]).normal().unwrap();

        let q = Rot::from_to(v1, v2);
        if (Vect::from(v2) - q.apl(Vect::from(v1))).magn() > 0.0001 {
            panic!("{v2} != {}", q.apl(Vect::from(v1)));
        }
    }
}
