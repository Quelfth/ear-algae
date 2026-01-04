use culit::culit;

use crate::{Mat, Nrml, Vect, op_wrapper::scs, traits::*};

mod vect3;

impl<S: Field> Vect<4, S> {
    #[culit]
    pub fn point(position: Vect<3, S>) -> Self {
        Vect([position[0], position[1], position[2], 1S])
    }

    #[culit]
    pub fn point_at_inf(direction: Nrml<3, S>) -> Self {
        Vect([direction[0], direction[1], direction[2], 0S])
    }
}

impl<S: Field> Mat<4, 4, S> {
    #[culit]
    pub fn affine(linear: Mat<3, 3, S>, translation: Vect<3, S>) -> Self {
        let [[xx, xy, xz], [yx, yy, yz], [zx, zy, zz]] = linear.0;
        let [x, y, z] = translation.0;
        Mat([
            [xx, xy, xz, x],
            [yx, yy, yz, y],
            [zx, zy, zz, z],
            [0S, 0S, 0S, 1S],
        ])
    }

    #[culit]
    pub fn linear(linear: Mat<3, 3, S>) -> Self {
        let [[xx, xy, xz], [yx, yy, yz], [zx, zy, zz]] = linear.0;
        Mat([
            [xx, xy, xz, 0S],
            [yx, yy, yz, 0S],
            [zx, zy, zz, 0S],
            [0S, 0S, 0S, 1S],
        ])
    }

    #[culit]
    pub fn translation(translation: Vect<3, S>) -> Self {
        let [x, y, z] = translation.0;
        Mat([
            [1S, 0S, 0S, x],
            [0S, 1S, 0S, y],
            [0S, 0S, 1S, z],
            [0S, 0S, 0S, 1S],
        ])
    }

    #[culit]
    pub fn perspective_projection(aspect: S, fov: S, n: S, f: S) -> Self {
        scs!(aspect, fov, f, n);
        let view = (fov * 0.5Sc).tan();

        Mat::from_scs([
            [1Sc / (aspect * view), 0Sc, 0Sc, 0Sc],
            [0Sc, 1Sc / view, 0Sc, 0Sc],
            [0Sc, 0Sc, ((f + n) / (n - f) - 1Sc) / 2Sc, f * n / (n - f)],
            [0Sc, 0Sc, -1Sc, 0Sc],
        ])
    }

    #[culit]
    pub fn uniform_scale(scale: S) -> Self {
        Mat([
            [scale, 0S, 0S, 0S],
            [0S, scale, 0S, 0S],
            [0S, 0S, scale, 0S],
            [0S, 0S, 0S, 1S],
        ])
    }
}
