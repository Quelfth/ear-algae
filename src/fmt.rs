use std::{fmt::*, ops::Index};

use culit::culit;

use crate::{
    Mat, Nrml, Rig, Rot, Vect,
    op_wrapper::{Sc, scs},
    rotor::{Axis, Bivector, RotDim},
    traits::{Field, Ring},
};

impl<S: Ring + Display, const N: usize> Display for Vect<N, S> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.write_str("{")?;
        for i in 0..N - 1 {
            f.write_str(&format!("{}, ", self[i]))?;
        }
        f.write_str(&format!("{}}}", self[N - 1]))
    }
}

impl<S: Ring + Debug, const N: usize> Debug for Vect<N, S> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.debug_list().entries(self.0).finish()
    }
}

impl<S: Field + Display, const N: usize> Display for Nrml<N, S> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.write_str("N{")?;
        for i in 0..N - 1 {
            f.write_str(&format!("{}, ", self[i]))?;
        }
        f.write_str(&format!("{}}}", self[N - 1]))
    }
}

impl<S: Field + Debug, const N: usize> Debug for Nrml<N, S> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "N")?;
        f.debug_list().entries(Vect::from(*self).0).finish()
    }
}

impl<S: Field + Display, const N: usize, const M: usize> Display for Mat<N, M, S> {
    fn fmt(&self, f: &mut Formatter) -> Result {
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

impl<const N: usize, S: Field + Display> Display for Rot<N, S>
where
    (): RotDim<N>,
    Bivector<N, S>: Index<usize, Output = S>,
    Axis<N, S>: Display,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        if N == 2 {
            let angle = self.angle();
            let sign = self.axis_or_zero()[0];
            scs!(angle, sign);
            write!(f, "{}π rad", angle / Sc::PI * sign)
        } else if let Some(axis) = self.axis() {
            let angle = self.angle();
            scs!(angle);
            write!(f, "{}π rad about {}", angle / Sc::PI, axis)
        } else {
            write!(f, "0rad")
        }
    }
}

impl<const N: usize, S: Field + Debug> Debug for Rot<N, S>
where
    (): RotDim<N>,
    Axis<N, S>: Debug,
{
    #[culit]
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Some(axis) = &self.axis() {
            f.debug_tuple("Rot")
                .field(&self.angle())
                .field(axis)
                .finish()
        } else {
            f.debug_tuple("Rot").field(&0S).finish()
        }
    }
}

impl<const N: usize, S: Field + Debug> Debug for Rig<N, S>
where
    (): RotDim<N>,
    Rot<N, S>: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.debug_struct("Rig")
            .field("trans", &self.trans)
            .field("rot", &self.rot)
            .finish()
    }
}

impl<S: Display> Display for Sc<S> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}
