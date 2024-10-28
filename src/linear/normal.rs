use std::{array, fmt::{self, Display}, ops::{Add, Div, Index, Mul, Neg, Sub}};

use crate::smath;

use super::{ops::{AngleTo, Cross, Dot, ProjRej}, restricted::Restricted, Field, OptionNrmlRelax, Vect};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Nrml<const N: usize, S: Field> (pub(crate) [S; N]);

impl<S: Field, const N: usize> Nrml<N, S> {
    /// .
    ///
    /// # Safety
    /// 
    /// This is safe if the values are actually normalized.
    /// 
    /// They should have just been divided by their magnitude.
    ///
    /// .
    pub const unsafe fn new_unchecked(array: [S; N]) -> Nrml<N, S> {
        Nrml(array)
    }
    

    pub const fn axis(i: usize) -> Self {
        let mut v = [S::ZERO; N];
        v[i] = S::ONE;
        Self(v)
    }

    

    // pub fn finite(self) -> FinVect<N, S> {
    //     FinVect(self.0)
    // }

    
}

impl<S: Field, const N: usize> From<Nrml<N, S>> for Vect<N, S> {
    fn from(value: Nrml<N, S>) -> Self {
        value.relax()
    }
}



impl<S: Field, const N:usize> Index<usize> for Nrml<N, S> {
    type Output = S;

    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl<S: Field, const N: usize> Nrml<N, S> {
    pub fn array(self) -> [S; N] {
        self.0
    }
}

impl<S: Field, const N: usize> Add for Nrml<N, S> {
    type Output = Vect<N, S>;
    
    fn add(self, rhs: Self) -> Self::Output {
        Vect::from_fn(|i| self[i].add(rhs[i]))
    }
}

impl<S: Field, const N: usize> Add<Vect<N, S>> for Nrml<N, S> {
    type Output = Vect<N, S>;
    
    fn add(self, rhs: Vect<N, S>) -> Self::Output {
        Vect::from_fn(|i| self[i].add(rhs[i]))
    }
}

impl<S: Field, const N: usize> Add<Nrml<N, S>> for Vect<N, S> {
    type Output = Vect<N, S>;
    
    fn add(self, rhs: Nrml<N, S>) -> Self::Output {
        Vect::from_fn(|i| self[i].add(rhs[i]))
    }
}

impl<S: Field, const N: usize> Sub for Nrml<N, S> {
    type Output = Vect<N, S>;
    
    fn sub(self, rhs: Self) -> Self::Output {
        Vect::from_fn(|i| self[i].sub(rhs[i]))
    }
}

impl<S: Field, const N: usize> Sub<Vect<N, S>> for Nrml<N, S> {
    type Output = Vect<N, S>;
    
    fn sub(self, rhs: Vect<N, S>) -> Self::Output {
        Vect::from_fn(|i| self[i].sub(rhs[i]))
    }
}

impl<S: Field, const N: usize> Sub<Nrml<N, S>> for Vect<N, S> {
    type Output = Vect<N, S>;
    
    fn sub(self, rhs: Nrml<N, S>) -> Self::Output {
        Vect::from_fn(|i| self[i].sub(rhs[i]))
    }
}

impl<S: Field, const N: usize> Neg for Nrml<N, S> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(array::from_fn(|i| self[i].neg()))
    }
}

impl<S: Field, const N: usize> Mul<S> for Nrml<N, S> {
    type Output = Vect<N, S>;

    fn mul(self, rhs: S) -> Self::Output {
        Vect::from_fn(|i| self[i].mul(rhs))
    }
}

impl<S: Field, const N: usize> Div<S> for Nrml<N, S> {
    type Output = Vect<N, S>;

    fn div(self, rhs: S) -> Self::Output {
        Vect::from_fn(|i| self[i].div(rhs))
    }
}

impl<S: Field, const N: usize> Dot<Self> for Nrml<N, S> {
    fn dot(self, other: Self) -> S {
        let dot = (0..N).map(|i| self[i].mul(other[i])).fold(S::ZERO, |c, n| c.add(n));
        if dot > S::ONE {
            S::ONE
        } else if dot < smath!{-1} {
            smath!{-1}
        } else {
            dot
        }
    }
    
    type Output = S;
}

impl<S: Field, const N: usize> Dot<Vect<N, S>> for Nrml<N, S> {
    fn dot(self, other: Vect<N, S>) -> S {
        self.relax().dot(other)
    }
    
    type Output = S;
}

impl<S: Field, const N: usize> Dot<Nrml<N, S>> for Vect<N, S> {
    fn dot(self, other: Nrml<N, S>) -> S {
        self.dot(other.relax())
    }
    
    type Output = S;
}


impl<S: Field, const N: usize> Cross for Nrml<N, S> where Vect<N, S>: Cross {
    type Output = <Vect<N, S> as Cross>::Output;

    fn cross(self, other: Self) -> Self::Output {
        self.relax().cross(other.relax())
    }
}

impl<S: Field, const N: usize> Cross<Vect<N, S>> for Nrml<N, S> where Vect<N, S>: Cross {
    type Output = <Vect<N, S> as Cross>::Output;

    fn cross(self, other: Vect<N, S>) -> Self::Output {
        self.relax().cross(other)
    }
}

impl<S: Field, const N: usize> Cross<Nrml<N, S>> for Vect<N, S> where Self: Cross {
    type Output = <Self as Cross>::Output;

    fn cross(self, other: Nrml<N, S>) -> Self::Output {
        self.cross(other.relax())
    }
}


impl<S: Field, const N: usize> AngleTo for Nrml<N, S> {
    type Output = S;

    fn angle_to(self, other: Self) -> Self::Output {
        self.dot(other).acos()
    }
}

impl<S: Field, const N: usize> AngleTo<Option<Self>> for Nrml<N, S> {
    type Output = S;

    fn angle_to(self, other: Option<Self>) -> Self::Output {
        match other {
            Some(other) => self.angle_to(other),
            None => S::ZERO
        }
    }
}

impl<S: Field, const N: usize> AngleTo<Nrml<N, S>> for Option<Nrml<N, S>> {
    type Output = S;

    fn angle_to(self, other: Nrml<N, S>) -> Self::Output {
        match self {
            Some(s) => s.angle_to(other),
            None => S::ZERO
        }
    }
}

impl<S: Field, const N: usize> AngleTo for Option<Nrml<N, S>> {
    type Output = S;

    fn angle_to(self, other: Self) -> Self::Output {
        match (self, other) {
            (Some(a), Some(b)) => a.angle_to(b),
            _ => S::ZERO
        }
    }
}

impl<S: Field, const N: usize> ProjRej<Nrml<N, S>> for Nrml<N, S> {
    type Output = Vect<N, S>;

    fn proj(self, axis: Nrml<N, S>) -> Vect<N, S> {
        axis * self.dot(axis)
    }

    fn rej(self, axis: Nrml<N, S>) -> Vect<N, S> {
        self - self.proj(axis)
    }

    fn proj_rej(self, axis: Nrml<N, S>) -> (Vect<N, S>, Vect<N, S>) {
        let proj = self.proj(axis);
        (proj, self - proj)
    }

}

impl<S: Field, const N: usize> ProjRej<Option<Nrml<N, S>>> for Nrml<N, S> {
    type Output = Vect<N, S>;

    fn proj(self, axis: Option<Nrml<N, S>>) -> Self::Output {
        match axis {
            Some(axis) => self.proj(axis),
            None => Vect::ZERO
        }
    }
    
    fn rej(self, axis: Option<Nrml<N, S>>) -> Self::Output {
        self - self.proj(axis)
    }

    fn proj_rej(self, axis: Option<Nrml<N, S>>) -> (Self::Output, Self::Output) {
        let proj = self.proj(axis);
        (proj, self - proj)
    }

}


impl<S: Field+Display, const N: usize> Display for Nrml<N, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("N{")?;
        for i in 0..N-1 {
            f.write_str(&format!("{}, ", self[i]))?;
        };
        f.write_str(&format!("{}}}", self[N-1]))
    }
}

impl<S: Field+fmt::Debug, const N: usize> fmt::Debug for Nrml<N, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "N")?;
        f.debug_list().entries(self.0).finish()
    }
}

impl<const N: usize, S: Field> OptionNrmlRelax<N, S> for Option<Nrml<N, S>> {
    fn relax_or(self, or: Vect<N, S>) -> Vect<N, S> {
        match self {
            None => or,
            Some(n) => n.relax()
        }
    }

    fn relax_or_zero(self) -> Vect<N,S> {
        self.relax_or(Vect::ZERO)
    }
}

impl<const N: usize, S: Field> Restricted<Vect<N, S>> for Nrml<N, S> {
    fn relax(self) -> Vect<N, S> {
        Vect(self.0)
    }
    
    fn restrict(value: Vect<N, S>) -> Option<Self> {
        value.normal()
    }
}