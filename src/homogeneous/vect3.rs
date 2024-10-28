use crate::vect;

use super::{Field, Nrml, Vect};



#[repr(transparent)]
pub struct HomoVect3<S: Field>(Vect<4, S>);

impl<S: Field> From<Vect<3, S>> for HomoVect3<S> {
    fn from(value: Vect<3, S>) -> Self {
        HomoVect3(vect!(value[0], value[1], value[2], S::ONE))
    }
}

impl<S: Field> From<Nrml<3, S>> for HomoVect3<S> {
    fn from(value: Nrml<3, S>) -> Self {
        HomoVect3(vect!(value[0], value[1], value[2], S::ZERO))
    }
}