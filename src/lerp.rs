use lerpify::LinearCombination;

use crate::{Vect, traits::Ring};

impl<const N: usize, S: Ring> LinearCombination<S> for Vect<N, S> {
    fn linear_combination<const L: usize>(terms: [(Self, S); L]) -> Self {
        terms.into_iter().fold(Vect::ZERO, |c, n| c + (n.0 * n.1))
    }
}
