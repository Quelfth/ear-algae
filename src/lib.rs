#![deny(unsafe_op_in_unsafe_fn)]

pub mod traits;
pub mod linear;
pub mod rotor;
pub mod homogeneous;
pub mod rigid;
pub mod prelude;
pub mod lerp;
//pub mod scalar;


#[macro_export]
macro_rules! smath {
    (0) => {S::ZERO};
    (1) => {S::ONE};
    (-1) => {S::ONE.neg()};
    (2) => {S::TWO};
    (-2) => {S::TWO.neg()};
    (1/2) => {S::HALF};
    (-1/2) => {S::HALF.neg()};
    (PI) => {S::PI};
    (-PI) => {S::PI.neg()};

    ($a:ident) => {
        $a
    };

    (($a:tt^2)) => {
        smath!($a).pow(2)
    };

    (($a:tt / 2)) => {
        smath!($a).mul(S::HALF)
    };

    ((1/$a:tt)) => {
        S::ONE.div(smath!($a))
    };

    ((-$a:tt)) => {
        smath!($a).neg()
    };


    (($a:tt + $b:tt + $c:tt)) => {
        smath!($a).add(smath!($b)).add(smath!($c))
    };
    
    (($a:tt + $b:tt)) => {
        smath!($a).add(smath!($b))
    };

    (($a:tt - $b:tt)) => {
        smath!($a).sub(smath!($b))
    };

    (($a:tt * $b:tt * $c:tt)) => {
        smath!($a).mul(smath!($b)).mul(smath!($c))
    };

    (($a:tt * $b:tt)) => {
        smath!($a).mul(smath!($b))
    };

    (($a:tt / $b:tt)) => {
        smath!($a).div(smath!($b))
    };

    (($a:tt . $f:ident )) => {
        smath!($a).$f()
    };

    ($($x:tt)*) => {
        smath!{($($x)*)}
    }
}





