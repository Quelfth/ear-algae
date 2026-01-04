#![allow(unused)]

pub mod integer {
    macro_rules! S {
        (0) => {
            S::ZERO
        };
        (1) => {
            S::ONE
        };
        (2) => {
            S::TWO
        };
    }
    pub(crate) use S;

    macro_rules! Sc {
        (0) => {
            $crate::op_wrapper::Sc(S::ZERO)
        };
        (1) => {
            $crate::op_wrapper::Sc(S::ONE)
        };
        (2) => {
            $crate::op_wrapper::Sc(S::TWO)
        };
    }
    pub(crate) use Sc;
}

pub mod float {
    macro_rules! S {
        (0.5) => {
            S::HALF
        };
    }
    pub(crate) use S;

    macro_rules! Sc {
        (0.5) => {
            $crate::op_wrapper::Sc(S::HALF)
        };
    }
    pub(crate) use Sc;
}
