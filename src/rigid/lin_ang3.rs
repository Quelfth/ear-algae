
use super::*;


impl<S: Field> Add for LinAng3<S> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {lin: self.lin + rhs.lin, ang: self.ang + rhs.ang}
    }
}
impl<S: Field> Sub for LinAng3<S> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {lin: self.lin - rhs.lin, ang: self.ang - rhs.ang}
    }
}
impl<S: Field> Mul<S> for LinAng3<S> {
    type Output = Self;

    fn mul(self, rhs: S) -> Self::Output {
        Self {lin: self.lin * rhs, ang: self.ang * rhs}
    }
}
impl<S: Field> Div<S> for LinAng3<S> {
    type Output = Self;

    fn div(self, rhs: S) -> Self::Output {
        Self {lin: self.lin / rhs, ang: self.ang / rhs}
    }
}

impl<S: Field> LinAng<3, S> for LinAng3<S> {
    
    type Axis = Vect<3, S>;
    type Rig = Rig3<S>;

    const ZERO: Self = Self {lin: Vect::ZERO, ang: Vect::ZERO};
    
    fn new(lin: Vect<3, S>, ang: Self::Axis) -> Self {
        Self {lin, ang}
    }

    fn lin(self) -> Vect<3, S> {self.lin}
    fn ang(self) -> Vect<3, S> {self.ang}
    
    fn rig(self) -> Rig3<S> {
        Rig3::new(
            self.lin,
            Rot3::from_ang(self.ang)
        )
    }
}
