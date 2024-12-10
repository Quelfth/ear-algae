use std::ops::*;

use serde::{Deserialize, Serialize};

use crate::{linear::*, rotor::*, traits::*};

use self::ops::{Apl, BefAft};

pub trait Rig<const N: usize, S: Field>: Sized+Copy+Apl<Vect<N,S>, Output = Vect<N,S>>+Apl<Nrml<N,S>, Output = Vect<N,S>>+BefAft {
    type Rot: Rot<N, S>;

    const IDENT: Self;

    fn new(vect: Vect<N, S>, rot: Self::Rot) -> Self;

    fn rot(self) -> Self::Rot;
    fn trans(self) -> Vect<N, S>;

    fn rot_mut(&mut self) -> &mut Self::Rot;
    fn trans_mut(&mut self) -> &mut Vect<N, S>;

    fn inv(self) -> Self;
}

pub trait LinAng<const N: usize, S: Field>: Sized+Copy+Add<Output = Self>+Sub<Output = Self>+Mul<S, Output = Self>+Div<S, Output = Self> {
    type Axis;
    type Rig: Rig<N, S>;

    const ZERO: Self;

    fn new(lin: Vect<N, S>, ang: Self::Axis) -> Self;
    

    fn lin(self) -> Vect<N, S>;
    fn ang(self) -> Self::Axis;

    fn rig(self) -> Self::Rig;
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Rig2<S: Field> {rot: Rot2<S>, trans: Vect<2, S>}
mod rig2;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Rig3<S: Field> {rot: Rot3<S>, trans: Vect<3, S>}
mod rig3;

#[derive(Copy, Clone, Debug)]
pub struct LinAng3<S: Field> {lin: Vect<3, S>, ang: Vect<3, S>}
mod lin_ang3;