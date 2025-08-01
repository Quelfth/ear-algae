use std::{array, fmt::{Display, Formatter}, ops::*};

use bytemuck::{AnyBitPattern, NoUninit, Pod, TransparentWrapper, Zeroable};

use crate::smath;

use self::ops::{Conv, Det, Dot};

use super::*;


#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, TransparentWrapper)]
pub struct Mat<const N: usize, const M: usize, S: Field> (pub(crate) [[S; M]; N]);

unsafe impl<const N: usize, const M: usize, S: Field> AnyBitPattern for Mat<N, M, S> where [[S; M]; N]: AnyBitPattern {}
unsafe impl<const N: usize, const M: usize, S: Field> Zeroable for Mat<N, M, S> where [[S; M]; N]: Zeroable {}

unsafe impl<const N: usize, const M: usize, S: Field> NoUninit for Mat<N, M, S> where [[S; M]; N]: NoUninit {}

impl<S: Field, const N: usize, const M: usize> Mat<N, M, S> {
    pub fn new(array: [[S; M]; N]) -> Self {
        Mat(array)
    }
}

impl<F: Field+Conv<T>, T: Field, const N: usize, const M: usize> Conv<Mat<N, M, T>> for Mat<N, M, F> {
    fn conv(self) -> Mat<N, M, T> {
        Mat(self.0.conv())
    }
}

#[macro_export]
macro_rules! mat {
    ($($($x:expr),*);*) => {
        Mat::new([$([$($x),*]),*])
    };
}

#[macro_export]
macro_rules! smath_mat {
    ($($($x:tt),*);*) => {
        Mat::new([$([$(smath!{$x}),*]),*])
    };
}

impl<S: Field, const N:usize, const M:usize> Mat<N, M, S> {
    pub fn try_index(&self, i:usize, j:usize) -> Option<S> {
        if i > N || j > M {
            None
        } else {
            Some(self[i][j])
        }
    }
}

impl<S: Field, const N: usize, const M: usize> Mat<N, M, S> {
    fn row_swap_assign(&mut self, i1: usize, i2: usize) {

            (self.0[i1], self.0[i2]) = (self.0[i2], self.0[i1])

    }

    fn row_sub_assign(&mut self, i1: usize, coefficient: S, i2: usize) {
        for j in 0..M {
            self.0[i1][j].sub_assign(self[i2][j].mul(coefficient));
        }
    }

    fn row_div_assign(&mut self, i: usize, divisor: S) {
        for j in 0..M {
            self.0[i][j].div_assign(divisor);
        }
    }
}

struct AugMat<const N: usize, const M1: usize, const M2: usize, S: Field> (pub Mat<N, M1, S>, pub Mat<N, M2, S>);


impl<S: Field, const N:usize, const M1: usize, const M2: usize> AugMat<N, M1, M2, S> {
    fn row_swap_assign(&mut self, i1:usize, i2:usize) {
        self.0.row_swap_assign(i1, i2);
        self.1.row_swap_assign(i1, i2);
    }

    fn row_sub_assign(&mut self, i1: usize, coefficient: S, i2: usize) {
        self.0.row_sub_assign(i1, coefficient, i2);
        self.1.row_sub_assign(i1, coefficient, i2);
    }

    fn row_div_assign(&mut self, i: usize, divisor: S) {
        self.0.row_div_assign(i, divisor);
        self.1.row_div_assign(i, divisor);
    }
}

impl<S: Field, const N: usize> Mat<N, N, S> {
    pub fn inverse(&self) -> Self {
        let mut aug = AugMat(*self, Self::IDENT);
        for j in 0..N {
            {
                let mut max = self[j][j].abs();
                let mut i_max: usize = j;
                for i in j + 1..N {
                    let value = self[i][j].abs();
                    if value > max {
                        max = value;
                        i_max = i;
                    }
                }
                if j != i_max {
                    aug.row_swap_assign(j, i_max);
                }
            }
            aug.row_div_assign(j, aug.0[j][j]);
            for i in j+1..N {
                let c = aug.0[i][j];
                aug.row_sub_assign(i, c, j);
            }
        }
        for i in (0..N-1).rev() {
            for j in i+1..N {
                aug.row_sub_assign(i, aug.0[i][j], j);
            }
        }
        aug.1
    }
}



impl<S: Field> Det for Mat<1, 1, S> {
    type Output = S;

    fn det(self) -> S {
        self[0][0]
    }
}



impl<S: Field> Det for Mat<2, 2, S> {
    type Output = S;

    fn det(self) -> S {
        self[0][0].mul(self[1][1]).sub(self[0][1].mul(self[1][0]))
    }
}

impl<S: Field> Det for Mat<3, 3, S>{
    type Output = S;
    fn det(self) -> S {
        let a = self[0][0];
        let b = self[0][1];
        let c = self[0][2];
        let d = self[1][0];
        let e = self[1][1];
        let f = self[1][2];
        let g = self[2][0];
        let h = self[2][1];
        let i = self[2][2];


         //      a.mul(e.mul(i))  .add (b.mul(f.mul(g))) .add (c.mul(d.mul(h))) 
         //.sub (c.mul(e.mul(g))) .sub (b.mul(d.mul(i))) .sub (a.mul(f.mul(h)))
        
        smath!{((a*e*i) + (b*f*g) + (c*d*h)) - ((c*e*g) + (b*d*i) + (a*f*h))}
    }
}

impl<S: Field, const N: usize, const M: usize> Default for Mat<N, M, S> {
    fn default() -> Self {
        Self::ZERO
    }
}

impl<S: Field, const N: usize, const M: usize> Mat<N, M, S> {
    pub fn from_fn<F: Fn(usize, usize) -> S>(f: F) -> Self {
        Mat(array::from_fn(|i| array::from_fn(|j| f(i,j))))
    }

    pub const ZERO: Self = Mat([[S::ZERO; M]; N]);
}

impl<S: Field, const N: usize> Mat<N, N, S> {
    pub const IDENT: Self = {
        let mut array = [[S::ZERO; N]; N];
        let mut i = 0;
        while i < N {
            array[i][i] = S::ONE;
            i += 1;
        }
        Self(array)
    };
}

impl<S: Field, const N: usize, const M: usize> Index<usize> for Mat<N, M, S> {
    type Output = [S; M];

    fn index(&self, i: usize) -> &Self::Output { &self.0[i] }
}

impl<S: Field, const N: usize, const M: usize> Mat<N, M, S> {
    pub fn row(&self, i: usize) -> Vect<M, S> {
        Vect::from_fn(|j| self[i][j])
    }

    pub fn col(&self, j: usize) -> Vect<N, S> {
        Vect::from_fn(|i| self[i][j])
    }
}

impl<S: Field, const N: usize, const M: usize> Mul<Vect<M, S>> for Mat<N, M, S>{
    type Output = Vect<N, S>;

    fn mul(self, vector: Vect<M, S>) -> Self::Output {
        Vect::from_fn(|i| self.row(i).dot(vector))
    }
}

impl<S: Field, const N: usize, const M: usize> Mul<Mat<N, M, S>> for Vect<N, S> {
    type Output = Vect<N, S>;

    fn mul(self, matrix: Mat<N, M, S>) -> Self::Output {
        Vect::from_fn(|j| self.dot(matrix.col(j)))
    }
}

impl<S: Field, const N: usize, const M: usize, const P: usize> Mul<Mat<M, P, S>> for Mat<N, M, S> {
    type Output = Mat<N, P, S>;

    fn mul(self, other: Mat<M, P, S>) -> Self::Output {
        Mat::from_fn(|i,j| self.row(i).dot(other.col(j)))
    }
}

impl<S: Field+Display, const N: usize, const M: usize> Display for Mat<N, M, S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if N == 0 {
            f.write_str("[]")?
        } else {
            f.write_str(&format!("[{}", self[0][0]))?;
            for j in 1..M {
                f.write_str(&format!(", {}", self[0][j]))?;
            }
            for i in 1..N {
                f.write_str(&format!("|{}", self[i][0]))?;
                for j in 1..M {
                    f.write_str(&format!(", {}", self[i][j]))?;
                }
            }
            f.write_str("]")?
        }
        Ok(())
    }
}

impl<S: Field> Mat<4, 4, S> {
    pub fn flatten(self) -> [S; 16] {
        array::from_fn(|i| self[i % 4][i / 4])
    }
}

impl<S: Field, const N: usize, const M: usize> Mul<S> for Mat<N, M, S> {
    type Output = Self;

    fn mul(self, scalar: S) -> Self::Output {
        Mat::from_fn(|i,j| self[i][j].mul(scalar))
    }
}

impl<S: Field, const N: usize, const M: usize> Add for Mat<N, M, S> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Mat::from_fn(|i,j| self[i][j].add(other[i][j]))
    }
}

impl<S: Field, const N: usize, const M: usize> Sub for Mat<N, M, S> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Mat::from_fn(|i,j| self[i][j].sub(other[i][j]))
    }
}

impl<S: Field, const N: usize, const M: usize> Neg for Mat<N, M, S> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Mat::from_fn(|i,j| self[i][j].neg())
    }
}
