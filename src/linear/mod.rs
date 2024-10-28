use crate::traits::*;


pub trait OptionNrmlRelax<const N: usize, S: Field> {
    fn relax_or(self, or: Vect<N, S>) -> Vect<N, S>;
    fn relax_or_zero(self) -> Vect<N, S>;
}


mod vector;
pub use vector::Vect;


mod normal;
pub use normal::Nrml;


mod matrix;
pub use matrix::Mat;


mod ortho;
pub use ortho::Ortho;