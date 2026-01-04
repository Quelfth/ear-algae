#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    Mat, Vect,
    ops::{Apl, BefAft},
    rotor::*,
    traits::*,
};

#[derive(Copy, Clone, Default)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(bound(
        serialize = "
            [S; N]: Serialize,
            Bivector<N, S>: Serialize,
        ",
        deserialize = " 
            [S; N]: Deserialize<'de>,
            Bivector<N, S>: Deserialize<'de>,
        ",
    ))
)]
pub struct Rig<const N: usize, S: Field>
where
    (): RotDim<N>,
{
    pub trans: Vect<N, S>,
    pub rot: Rot<N, S>,
}

impl<const N: usize, S: Field> Rig<N, S>
where
    (): RotDim<N>,
{
    pub const IDENT: Self = Self {
        trans: Vect::ZERO,
        rot: Rot::IDENT,
    };

    pub fn new(trans: Vect<N, S>, rot: Rot<N, S>) -> Self {
        Self { trans, rot }
    }

    pub fn rot(rot: Rot<N, S>) -> Self {
        Self { rot, ..Self::IDENT }
    }

    pub fn trans(trans: Vect<N, S>) -> Self {
        Self {
            trans,
            ..Self::IDENT
        }
    }

    pub fn inv(self) -> Self {
        let rot = self.rot.inv();
        Self {
            rot,
            trans: rot.apl(-self.trans),
        }
    }
}

impl<const N: usize, S: Field> From<Rot<N, S>> for Rig<N, S>
where
    (): RotDim<N>,
{
    fn from(value: Rot<N, S>) -> Self {
        Self::rot(value)
    }
}

impl<const N: usize, S: Field> Apl<Vect<N, S>> for Rig<N, S>
where
    (): RotDim<N>,
{
    type Output = Vect<N, S>;

    fn apl(self, other: Vect<N, S>) -> Vect<N, S> {
        self.rot.apl(other) + self.trans
    }
}

impl<const N: usize, S: Field> BefAft for Rig<N, S>
where
    (): RotDim<N>,
{
    fn aft(self, other: Self) -> Self {
        Self {
            rot: self.rot.aft(other.rot),
            trans: self.rot.apl(other.trans) + self.trans,
        }
    }
}

impl<S: Field> Rig<3, S> {
    pub fn to_hmat(self) -> Mat<4, 4, S> {
        Mat::affine(self.rot.mat(), self.trans)
    }
}
