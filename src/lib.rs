extern crate num;

#[macro_use]
mod macros;

#[macro_use]
mod approx_eq;
mod clamp;
mod vector3;
mod vector4;
mod quaternion;
mod matrix4x4;

pub mod consts;
pub use approx_eq::ApproxEq;
pub use clamp::{Clamp, Clamp01};
pub use vector3::Vector3;
pub use vector4::Vector4;
pub use quaternion::Quaternion;
pub use matrix4x4::Matrix4x4;

#[cfg(test)]
mod tests;