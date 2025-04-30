

use crate::{linear::*, smath, smath_mat, traits::*, vect};

mod vect3;


impl<S: Field> Vect<4, S> {
    pub fn point(position: Vect<3, S>) -> Self {
        vect![position[0], position[1], position[2], S::ONE]
    }

    pub fn point_at_inf(direction: Nrml<3, S>) -> Self {
        vect![direction[0], direction[1], direction[2], S::ZERO]
    }
}






impl<S: Field> Mat<4, 4, S> {

    pub fn affine(linear: Mat<3, 3, S>, translation: Vect<3, S>) -> Self{
        let [[xx, xy, xz], [yx, yy, yz], [zx, zy, zz]] = linear.0;
        let [x, y, z] = translation.0;
        smath_mat![
            xx, xy, xz,  x;
            yx, yy, yz,  y;
            zx, zy, zz,  z;
            0,   0,  0,  1
        ]
    }

    pub fn linear(linear: Mat<3, 3, S>) -> Self {
        let [[xx, xy, xz], [yx, yy, yz], [zx, zy, zz]] = linear.0;
        smath_mat![
            xx, xy, xz,  0;
            yx, yy, yz,  0;
            zx, zy, zz,  0;
             0,  0,  0,  1
        ]
    }

    pub fn translation(translation: Vect<3, S>) -> Self{
        let [x, y, z] = translation.0;
        smath_mat![
            1, 0, 0, x;
            0, 1, 0, y;
            0, 0, 1, z;
            0, 0, 0, 1
        ]
    }

    pub fn perspective_projection(aspect: S, fov: S, n: S, f: S) -> Self {
        let view = smath!{( (fov / 2).tan )};
    
        smath_mat![
            (1/(aspect * view)),        0,                     0,               0;
            0,                   (1/view),                     0,               0;
            0,                          0, ((((f+n)/(n-f))-1)/2), ((f*n) / (n-f));
            0,                          0,                  (-1),               0
        ]
    }

    pub fn uniform_scale(scale: S) -> Self {
        smath_mat![
            scale,     0,     0,     0;
                0, scale,     0,     0;
                0,     0, scale,     0;
                0,     0,     0,     1
        ]
    }
}
