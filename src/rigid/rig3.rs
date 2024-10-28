use std::fmt::Display;

use crate::prelude::Restricted;

use super::{ops::{Apl, BefAft}, Field, Mat, Nrml, Rig, Rig3, Rot, Rot3, Vect};




impl<S: Field> Rig<3, S> for Rig3<S> {
    type Rot = Rot3<S>;

    const IDENT: Self = Self{rot: Rot3::IDENT, trans: Vect::ZERO};

    fn new(trans: Vect<3, S>, rot: Self::Rot) -> Self {
        Self {rot, trans}
    }
    
    fn rot(self) -> Self::Rot {
        self.rot
    }
    
    fn trans(self) -> Vect<3, S> {
        self.trans
    }

    fn rot_mut(&mut self) -> &mut Self::Rot {
        &mut self.rot
    }
    
    fn trans_mut(&mut self) -> &mut Vect<3, S> {
        &mut self.trans
    }

    fn inv(self) -> Self {
        let rot = self.rot.inv();
        Self {
            rot,
            trans: rot.apl(-self.trans)
        }
    }
}

impl<S: Field> From<Rot3<S>> for Rig3<S> {
    fn from(value: Rot3<S>) -> Self {
        Self::new(Vect::ZERO, value)
    }
}

// impl<S: Field> Apl<Self> for Rig3<S> {
//     type Output = Self;

//     fn apl(self, other: Self) -> Self::Output {
//         Self {
//             rot: self.rot.aft(other.rot),
//             trans: self.rot.apl(other.trans) + self.trans
//         }
//     }
// }

impl<S: Field> Apl<Vect<3, S>> for Rig3<S> {
    type Output = Vect<3, S>;

    fn apl(self, other: Vect<3, S>) -> Self::Output {
        self.rot.apl(other) + self.trans
    }
}

impl<S: Field> Apl<Nrml<3, S>> for Rig3<S> {
    type Output = Vect<3, S>;

    fn apl(self, other: Nrml<3, S>) -> Self::Output {
        self.rot.apl(other.relax()) + self.trans
    }
}

impl<S: Field> BefAft for Rig3<S> {
    fn aft(self, other: Self) -> Self {
        Self {
            rot: self.rot.aft(other.rot),
            trans: self.rot.apl(other.trans) + self.trans
        }
    }
}

impl<S: Field> Rig3<S> {
    pub fn to_hmat(self) -> Mat<4, 4, S> {
        Mat::affine(self.rot.mat(), self.trans)
    }
}

impl<S: Field+Display> Display for Rig3<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("[{} & {}]", self.trans, self.rot))
    }
}