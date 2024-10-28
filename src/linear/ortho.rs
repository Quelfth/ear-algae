use std::{array, iter, ops::Index};

use crate::rotor::Rot;

use super::{ops::{Det, ProjRej}, Field, Mat, Nrml, Vect};

#[derive(Copy, Clone, PartialEq)]
pub struct Ortho<const N: usize, S: Field>(pub(crate) [Nrml<N, S>; N]);

impl<const N: usize, S: Field> Ortho<N, S> where Mat<N, N, S>: Det<Output = S>{
    pub fn from_vects(vectors: impl IntoIterator<Item = Vect<N, S>>) -> Self {
        Self::from_iter(vectors.into_iter().filter_map(Vect::normal))
    }

    pub fn from_nrmls(normals: impl IntoIterator<Item = Nrml<N, S>>) -> Self {
        Self::from_iter(normals.into_iter())
    }

    fn from_iter(normals: impl Iterator<Item = Nrml<N, S>>) -> Self {
        let mut orthos = array::from_fn(|i| Nrml::axis(N-i));
        let mut i = N as isize;

        let iter_a = normals.map(Some).chain(iter::once(None).cycle());
        let iter_b = (0..N).rev().map(Nrml::axis).cycle();

        let iter = iter_a.zip(iter_b).map(|(a, b)| a.unwrap_or(b));


        'normals:
        for mut normal in iter {
            if i < 0 {
                break;
            }
            
            for ortho in orthos {
                let Some(rej) = normal.rej(ortho).normal() else {
                    continue 'normals;
                };
                normal = rej;
            }

            orthos[i as usize] = normal;
            i -= 1;
        }

        let mut ortho = Self(orthos);

        if ortho.as_rows().det() < S::ZERO {
            ortho.0[0] = -ortho.0[0];
        }

        ortho
    }


    
}

impl<const N: usize, S: Field> Default for Ortho<N, S> {
    fn default() -> Self {
        Self(array::from_fn(Nrml::axis))
    }
}

impl<const N: usize, S: Field> Ortho<N, S> {

    pub fn as_rows(self) -> Mat<N, N, S> {
        Mat::from_fn(|i, j| self.0[i][j])
    }

    pub fn rot<R: Rot<N, S>>(self) -> R {
        R::from_ortho(self)
    }
}

impl<const N: usize, S: Field, I> Index<I> for Ortho<N, S> where [Nrml<N, S>; N]: Index<I> {
    type Output = <[Nrml<N, S>; N] as Index<I>>::Output;

    fn index(&self, i: I) -> &Self::Output {
        &self.0[i]
    }
}

