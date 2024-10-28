use crate::vect;
use super::*;
use super::ops::*;


impl<S: Field> Cross for Vect<2, S> {
    type Output = Vect<1, S>;

    fn cross(self, other: Self) -> Self::Output {
        vect![
            self[0].mul(other[1]).sub(other[0].mul(self[1]))
        ]
    }
}

impl<S: Field> Cross for Vect<3, S> {
    type Output = Vect<3, S>;

    fn cross(self, other: Self) -> Self::Output {
        vect![
            self[1].mul(other[2]).sub(other[1].mul(self[2])),
            self[2].mul(other[0]).sub(other[2].mul(self[0])),
            self[0].mul(other[1]).sub(other[0].mul(self[1]))
        ]
    }
}

