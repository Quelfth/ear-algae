#![deny(unsafe_op_in_unsafe_fn)]
#![warn(unused_qualifications)]

mod custom_literal;
mod fmt;
mod homogeneous;
#[cfg(feature = "lerpify")]
mod lerp;
pub mod matrix;
pub mod normal;
mod op_wrapper;
pub mod ops;
pub mod prelude;
pub mod rigid;
pub mod rotor;
pub mod traits;
pub mod vector;

pub use {matrix::Mat, normal::Nrml, rigid::Rig, rotor::Rot, vector::Vect};
