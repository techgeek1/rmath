#[macro_use]
mod macros;

mod approx_eq;
pub use approx_eq::ApproxEq;

mod clamp;
pub use clamp::{Clamp, Clamp01};

mod vector3;
pub use vector3::Vector3;

mod quaternion;
pub use quaternion::Quaternion;

mod matrix4x4;
pub use matrix4x4::Matrix4x4;