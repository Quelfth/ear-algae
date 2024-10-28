


pub trait Conv<O> {
    fn conv(self) -> O;
}

pub trait Dot<T> {
    type Output;

    fn dot(self, other: T) -> Self::Output; 
}

pub trait Cross<T = Self> {
    type Output;

    fn cross(self, other: T) -> Self::Output;
}

pub trait Apl<T> {
    type Output;

    fn apl(self, other: T) -> Self::Output;
}

pub trait BefAft: Sized+Copy {
    fn aft(self, other: Self) -> Self;
    fn bef(self, other: Self) -> Self {other.aft(self)}

    fn aft_assign(&mut self, other: Self) {
        *self = self.aft(other)
    }
    fn bef_assign(&mut self, other: Self) {
        *self = self.bef(other)
    }
}

pub trait ProjRej<A>: Sized {
    type Output;
    fn proj(self, axis: A) -> Self::Output;
    fn rej(self, axis: A) -> Self::Output;
    fn proj_rej(self, axis: A) -> (Self::Output, Self::Output);
}

pub trait Refl<A> {
    type Output;
    fn refl(self, axis: A) -> Self::Output;
}

pub trait AngleTo<T = Self> {
    type Output;
    fn angle_to(self, other: T) -> Self::Output;
}




pub trait Det {
    type Output;
    fn det(self) -> Self::Output;
}

pub trait Aplable<A> {
    type Output;
    fn apply(self, apler: A) -> Self::Output;
}

impl<A, T: Aplable<A>> Apl<T> for A {
    type Output = <T as Aplable<A>>::Output;

    fn apl(self, other: T) -> Self::Output {
        other.apply(self)
    }
}