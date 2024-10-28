
use std::cmp::Ordering;

use super::{Field, Ring};

macro_rules! unsigned_impls {
    ($($u:ty),*) => {
        $(
        impl Ring for $u {
            const ZERO: Self = 0;
            const ONE: Self = 1;
            const TWO: Self = 2;
            fn add(self, other: Self) -> Self {self + other}
            fn add_assign(&mut self, other: Self) { *self += other }
            fn mul(self, other: Self) -> Self {self * other}
            fn mul_assign(&mut self, other: Self) { *self *= other }
            fn sub(self, other: Self) -> Self {self-other}
            fn sub_assign(&mut self, other: Self) { *self -= other }
            fn div(self, other: Self) -> Self {self/other}
            fn div_assign(&mut self, other: Self) { *self /= other }
            fn pow(self, other: u32) -> Self {self.pow(other)}
            fn abs(self) -> Self {self}
            fn sign(self) -> Self {if self == 0 {0} else {1}}
            fn neg(self) -> Self {0-self}
            fn cmp(&self, other: &Self) -> Ordering {Ord::cmp(self, other)}
            fn is_zero(self) -> bool {self == 0}
            fn is_nan(self) -> bool {false}
            fn is_finite(self) -> bool {true}
            fn rem_euclid(self, other: Self) -> Self {self % other}
        })*
    }
}

macro_rules! signed_impls {
    ($($i:ty),*) => {
        $(
        impl Ring for $i {
            const ZERO: Self = 0;
            const ONE: Self = 1;
            const TWO: Self = 2;
            fn add(self, other: Self) -> Self {self + other}
            fn add_assign(&mut self, other: Self) { *self += other }
            fn mul(self, other: Self) -> Self {self * other}
            fn mul_assign(&mut self, other: Self) { *self *= other }
            fn sub(self, other: Self) -> Self {self-other}
            fn sub_assign(&mut self, other: Self) { *self -= other }
            fn div(self, other: Self) -> Self {self/other}
            fn div_assign(&mut self, other: Self) { *self /= other }
            fn pow(self, other: u32) -> Self {self.pow(other)}
            fn abs(self) -> Self {self.abs()}
            fn sign(self) -> Self {match Ord::cmp(&self, &0) {Ordering::Less => {-1}, Ordering::Equal => {0}, Ordering::Greater => {1}}}
            fn neg(self) -> Self {-self}
            fn cmp(&self, other: &Self) -> Ordering {Ord::cmp(self, other)}
            fn is_zero(self) -> bool {self == 0}
            fn is_nan(self) -> bool {false}
            fn is_finite(self) -> bool {true}
            fn rem_euclid(self, other: Self) -> Self {self.rem_euclid(other)}
        })*
    }
}

macro_rules! float_impls {
    ($($f:ty: $l:ident),*) => {
        $(
        impl Ring for $f {
            const ZERO: Self = 0.;
            const ONE: Self = 1.;
            const TWO: Self = 2.;
            fn add(self, other: Self) -> Self {self + other}
            fn add_assign(&mut self, other: Self) { *self += other }
            fn mul(self, other: Self) -> Self {self * other}
            fn mul_assign(&mut self, other: Self) { *self *= other }
            fn sub(self, other: Self) -> Self {self-other}
            fn sub_assign(&mut self, other: Self) { *self -= other }
            fn div(self, other: Self) -> Self {self/other}
            fn div_assign(&mut self, other: Self) { *self /= other }
            fn pow(self, other: u32) -> Self {self.powi(other as i32)}
            fn abs(self) -> Self {self.abs()}
            fn sign(self) -> Self {match self.float_cmp(&0.) {Ordering::Less => {-1.}, Ordering::Equal => {0.}, Ordering::Greater => {1.}}}
            fn neg(self) -> Self {-self}
            fn cmp(&self, other: &Self) -> Ordering {self.float_cmp(&other)}
            fn is_zero(self) -> bool {self == 0.}
            fn is_nan(self) -> bool {self.is_nan()}
            fn is_finite(self) -> bool {self.is_finite()}
            fn rem_euclid(self, other: Self) -> Self {self.rem_euclid(other)}
        }
    
        impl Field for $f {
            const HALF: Self = 0.5;
        
            const PI: Self = std::$l::consts::PI;

            const INFINITY: Self = Self::INFINITY;

            fn sqrt(self) -> Self {self.sqrt()}
        
            fn exp(self) -> Self {self.exp()}
        
            fn sin(self) -> Self {self.sin()}
            fn cos(self) -> Self {self.cos()}
            fn tan(self) -> Self {self.tan()}
        
            fn sin_cos(self) -> (Self, Self) {self.sin_cos()}
        
            fn ln(self) -> Self {self.ln()}
        
            fn asin(self) -> Self {self.asin()}
            fn acos(self) -> Self {self.acos()}
            fn atan(self) -> Self {self.atan()}
        
            fn atan2(y: Self, x: Self) -> Self {Self::atan2(y, x)}
        })*
    }
}

unsigned_impls!{u8, u16, u32, u64, u128, usize}
signed_impls!{i8, i16, i32, i64, i128, isize}
float_impls!{f32: f32, f64: f64}



impl Ring for bool {
    const ZERO: Self = false;
    const ONE: Self = true;
    const TWO: Self = true;
    fn add(self, other: Self) -> Self {self || other}
    fn add_assign(&mut self, other: Self) { *self |= other }
    fn mul(self, other: Self) -> Self {self && other}
    fn mul_assign(&mut self, other: Self) { *self &= other }
    fn sub(self, other: Self) -> Self {self != other}
    fn sub_assign(&mut self, other: Self) { *self ^= other }
    fn div(self, other: Self) -> Self {self == other}
    fn div_assign(&mut self, other: Self) { *self ^= !other }
    fn pow(self, other: u32) -> Self {self || other==0}
    fn abs(self) -> Self {self}
    fn sign(self) -> Self {self}
    fn neg(self) -> Self {self}
    fn cmp(&self, other: &Self) -> Ordering {Ord::cmp(self, other)}
    fn is_zero(self) -> bool {!self}
    fn is_nan(self) -> bool {false}
    fn is_finite(self) -> bool {true}
    fn rem_euclid(self, _: Self) -> Self {false}
}



trait FloatCmp {
    fn float_cmp(&self, other: &Self) -> Ordering;
}

impl FloatCmp for f32 {
    fn float_cmp(&self, other: &Self) -> Ordering {
        if let Some(cmp) = self.partial_cmp(other) {
            return cmp;
        }
    
        if self.is_nan() {
            if other.is_nan() {
                return Ordering::Equal;
            }
            return Ordering::Less;
        }
        Ordering::Greater
    }
}

impl FloatCmp for f64 {
    fn float_cmp(&self, other: &Self) -> Ordering {
        if let Some(cmp) = self.partial_cmp(other) {
            return cmp;
        }
    
        if self.is_nan() {
            if other.is_nan() {
                return Ordering::Equal;
            }
            return Ordering::Less;
        }
        Ordering::Greater
    }
}