use std::{array, iter::Sum, ops::*};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    Nrml,
    ops::{Dot, ProjRej, Refl},
    traits::{Field, Ring},
};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(
        bound(
            serialize = "[S; N]:  Serialize",
            deserialize = "[S; N]: Deserialize<'de>",
        ),
        transparent,
    )
)]
pub struct Vect<const N: usize, S>(pub [S; N]);

impl<const N: usize, S: Ring> Default for Vect<N, S> {
    fn default() -> Self {
        Self::ZERO
    }
}

impl<S: Ring, const N: usize> Vect<N, S> {
    pub const ZERO: Self = Vect([S::ZERO; N]);

    pub const fn axis(i: usize, value: S) -> Self {
        let mut v = [S::ZERO; N];
        v[i] = value;
        Self(v)
    }

    pub const fn splat(s: S) -> Self {
        Self([s; N])
    }

    pub fn from_fn<F: Fn(usize) -> S>(f: F) -> Self {
        Vect(array::from_fn(f))
    }

    pub fn swiz<const M: usize>(self, f: impl Fn([S; N]) -> [S; M]) -> Vect<M, S> {
        Vect(f(self.0))
    }

    pub fn map<T: Ring>(self, f: impl Fn(S) -> T) -> Vect<N, T> {
        Vect(self.0.map(f))
    }

    pub fn scale(self, other: Self) -> Self {
        Vect::from_fn(|i| self[i].mul(other[i]))
    }

    pub fn zero_extend<const M: usize>(self, i: usize) -> Vect<M, S>
    where
        S: Sized,
    {
        const { assert!(M > N) }
        Vect::<M, S>::from_fn(|j| {
            if j < i {
                self[i]
            } else if j > i + M - N {
                self[j - M - N]
            } else {
                S::ZERO
            }
        })
    }

    pub fn is_nan(self) -> bool {
        self.0.into_iter().any(S::is_nan)
    }

    pub fn is_finite(self) -> bool {
        self.0.into_iter().all(S::is_finite)
    }
}

impl<const N: usize> Vect<N, f32> {
    pub fn to_f64(self) -> Vect<N, f64> {
        self.map(|x| x as _)
    }
}

impl<const N: usize> Vect<N, f64> {
    pub fn to_f32(self) -> Vect<N, f32> {
        self.map(|x| x as _)
    }
}

impl<S: Ring, const N: usize> Index<usize> for Vect<N, S> {
    type Output = S;

    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl<S: Ring, const N: usize> IndexMut<usize> for Vect<N, S> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.0[i]
    }
}

impl<S: Ring, const N: usize> Index<RangeFull> for Vect<N, S> {
    type Output = [S];

    fn index(&self, index: RangeFull) -> &Self::Output {
        &self.0[index]
    }
}

impl<S: Ring, const N: usize> Add for Vect<N, S> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_fn(|i| self[i].add(rhs[i]))
    }
}

impl<S: Ring, const N: usize> Sub for Vect<N, S> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_fn(|i| self[i].sub(rhs[i]))
    }
}

impl<S: Ring, const N: usize> Neg for Vect<N, S> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::from_fn(|i| self[i].neg())
    }
}

impl<S: Ring, const N: usize> Mul<S> for Vect<N, S> {
    type Output = Self;

    fn mul(self, rhs: S) -> Self::Output {
        Self::from_fn(|i| self[i].mul(rhs))
    }
}

impl<S: Ring, const N: usize> Div<S> for Vect<N, S> {
    type Output = Self;

    fn div(self, rhs: S) -> Self::Output {
        Self::from_fn(|i| self[i].div(rhs))
    }
}

impl<S: Ring, const N: usize> Dot<Self> for Vect<N, S> {
    type Output = S;

    fn dot(self, other: Self) -> Self::Output {
        (0..N)
            .map(|i| self[i].mul(other[i]))
            .fold(S::ZERO, |c, n| c.add(n))
    }
}

impl<S: Ring, const N: usize> Vect<N, S> {
    pub fn sqr_magn(self) -> S {
        self.dot(self)
    }
}

impl<S: Field, const N: usize> Vect<N, S> {
    pub fn magn(self) -> S {
        self.sqr_magn().sqrt()
    }

    pub fn normal(self) -> Option<Nrml<N, S>> {
        Some(self.magn_normal()?.1)
    }

    pub fn normal_or_zero(self) -> Vect<N, S> {
        if let Some(normal) = self.normal() {
            normal.into()
        } else {
            Vect::ZERO
        }
    }

    pub fn magn_normal(self) -> Option<(S, Nrml<N, S>)> {
        if !self.is_finite() {
            if let Some(value) = self.divide_by_infinity() {
                return Some((S::INFINITY, unsafe { Nrml::new_unchecked(value.0) }));
            }
            return None;
        }
        let magn = self.magn();
        if magn.is_zero() {
            None
        } else {
            Some((magn, unsafe { Nrml::new_unchecked((self / magn).0) }))
        }
    }

    pub fn magn_normal_or_zero(self) -> (S, Vect<N, S>) {
        if let Some((magn, normal)) = self.magn_normal() {
            return (magn, normal.into());
        }
        (S::ZERO, Vect::ZERO)
    }

    pub fn divide_by_infinity(self) -> Option<Vect<N, S>> {
        if self.is_nan() {
            return None;
        }
        let mut array = self.0;
        let mut magn = S::ZERO;
        for e in &mut array {
            if e.is_finite() {
                *e = S::ZERO;
            } else {
                *e = e.sign();
                magn.add_assign(S::ONE);
            }
        }
        magn = magn.sqrt();

        let vect = Vect(array) / magn;

        if vect.is_finite() {
            return Some(vect);
        }
        None
    }
}

impl<S: Ring, const N: usize> Sum for Vect<N, S> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Vect::ZERO, |c, n| c + n)
    }
}

impl<S: Ring, const N: usize> AddAssign for Vect<N, S> {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self[i].add_assign(rhs[i])
        }
    }
}

impl<S: Ring, const N: usize> SubAssign for Vect<N, S> {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self[i].sub_assign(rhs[i])
        }
    }
}

impl<S: Ring, const N: usize> MulAssign<S> for Vect<N, S> {
    fn mul_assign(&mut self, rhs: S) {
        for i in 0..N {
            self[i].mul_assign(rhs)
        }
    }
}

impl<S: Ring, const N: usize> DivAssign<S> for Vect<N, S> {
    fn div_assign(&mut self, rhs: S) {
        for i in 0..N {
            self[i].div_assign(rhs)
        }
    }
}

impl<S: Field, const N: usize> ProjRej<Nrml<N, S>> for Vect<N, S> {
    type Output = Self;

    fn proj(self, axis: Nrml<N, S>) -> Self {
        axis * self.dot(axis)
    }

    fn rej(self, axis: Nrml<N, S>) -> Self {
        self - self.proj(axis)
    }

    fn proj_rej(self, axis: Nrml<N, S>) -> (Self, Self) {
        let proj = self.proj(axis);
        (proj, self - proj)
    }
}

impl<S: Field, const N: usize> ProjRej<Option<Nrml<N, S>>> for Vect<N, S> {
    type Output = Self;

    fn proj(self, axis: Option<Nrml<N, S>>) -> Self {
        match axis {
            Some(axis) => self.proj(axis),
            None => Vect::ZERO,
        }
    }

    fn rej(self, axis: Option<Nrml<N, S>>) -> Self {
        self - self.proj(axis)
    }

    fn proj_rej(self, axis: Option<Nrml<N, S>>) -> (Self, Self) {
        let proj = self.proj(axis);
        (proj, self - proj)
    }
}

impl<S: Field, const N: usize> ProjRej<Vect<N, S>> for Vect<N, S> {
    type Output = Self;

    fn proj(self, axis: Vect<N, S>) -> Self::Output {
        let magn2 = axis.sqr_magn();
        if magn2.is_zero() {
            Vect::ZERO
        } else {
            axis * self.dot(axis).div(magn2)
        }
    }

    fn rej(self, axis: Vect<N, S>) -> Self::Output {
        self - self.proj(axis)
    }

    fn proj_rej(self, axis: Vect<N, S>) -> (Self::Output, Self::Output) {
        let proj = self.proj(axis);
        (proj, self - proj)
    }
}

impl<S: Field, const N: usize> Refl<Nrml<N, S>> for Vect<N, S> {
    type Output = Self;

    fn refl(self, axis: Nrml<N, S>) -> Self::Output {
        self - self.proj(axis) * S::TWO
    }
}

impl<S: Field, const N: usize> Refl<Option<Nrml<N, S>>> for Vect<N, S> {
    type Output = Self;

    fn refl(self, axis: Option<Nrml<N, S>>) -> Self::Output {
        self - self.proj(axis) * S::TWO
    }
}

impl<S: Field, const N: usize> Refl<Vect<N, S>> for Vect<N, S> {
    type Output = Self;

    fn refl(self, axis: Vect<N, S>) -> Self::Output {
        self - self.proj(axis) * S::TWO
    }
}

impl<S: Field> Vect<1, S> {
    pub fn x(self) -> S {
        self[0]
    }
}

impl<S: Field> Vect<2, S> {
    pub fn x(self) -> S {
        self[0]
    }

    pub fn y(self) -> S {
        self[1]
    }
}

impl<S: Field> Vect<3, S> {
    pub fn x(self) -> S {
        self[0]
    }

    pub fn y(self) -> S {
        self[1]
    }

    pub fn z(self) -> S {
        self[2]
    }
}

impl<S: Field> Vect<4, S> {
    pub fn x(self) -> S {
        self[0]
    }

    pub fn y(self) -> S {
        self[1]
    }

    pub fn z(self) -> S {
        self[2]
    }

    pub fn w(self) -> S {
        self[3]
    }
}
