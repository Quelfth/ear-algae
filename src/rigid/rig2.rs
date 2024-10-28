use std::fmt::Display;

use crate::prelude::Restricted;

use super::{ops::{Apl, BefAft}, Field, Nrml, Rig, Rig2, Rot, Rot2, Vect};

impl<S: Field> Rig<2, S> for Rig2<S> {
    type Rot = Rot2<S>;

    const IDENT: Self = Self{rot: Rot2::IDENT, trans: Vect::ZERO};

    fn new(trans: Vect<2, S>, rot: Self::Rot) -> Self {
        Self {rot, trans}
    }
    
    fn rot(self) -> Self::Rot {
        self.rot
    }
    
    fn trans(self) -> Vect<2, S> {
        self.trans
    }

    fn rot_mut(&mut self) -> &mut Self::Rot {
        &mut self.rot
    }
    
    fn trans_mut(&mut self) -> &mut Vect<2, S> {
        &mut self.trans
    }

    fn inv(self) -> Self {
        Self {
            rot: self.rot.inv(),
            trans: -self.trans
        }
    }
}

impl<S: Field> Apl<Nrml<2, S>> for Rig2<S> {
    type Output = Vect<2, S>;

    fn apl(self, other: Nrml<2, S>) -> Self::Output {
        self.rot.apl(other.relax()) + self.trans
    }
}

impl<S: Field> Apl<Vect<2, S>> for Rig2<S> {
    type Output = Vect<2, S>;

    fn apl(self, other: Vect<2, S>) -> Self::Output {
        self.rot.apl(other) + self.trans
    }
}

impl<S: Field> BefAft for Rig2<S> {
    fn aft(self, other: Self) -> Self {
        Self {
            rot: self.rot.aft(other.rot),
            trans: self.rot.apl(other.trans) + self.trans
        }
    }
}

impl<S: Field+Display> Display for Rig2<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("[{} & {}]", self.trans, self.rot))
    }
}