use std::ops::{ Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign };
use std::cmp::{ PartialEq, Eq };
use std::fmt;

use { ApproxEq, Vector4, Quaternion };

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Matrix4x4 {
  pub m00: f32, pub m01: f32, pub m02: f32, pub m03: f32,
  pub m10: f32, pub m11: f32, pub m12: f32, pub m13: f32,
  pub m20: f32, pub m21: f32, pub m22: f32, pub m23: f32,
  pub m30: f32, pub m31: f32, pub m32: f32, pub m33: f32
}

#[allow(dead_code)]
impl Matrix4x4 {
    const ZERO: Matrix4x4 = Matrix4x4 {
        m00: 0.0, m01: 0.0, m02: 0.0, m03: 0.0,
        m10: 0.0, m11: 0.0, m12: 0.0, m13: 0.0,
        m20: 0.0, m21: 0.0, m22: 0.0, m23: 0.0,
        m30: 0.0, m31: 0.0, m32: 0.0, m33: 0.0
    };

    const IDENTITY: Matrix4x4 = Matrix4x4 {
        m00: 1.0, m01: 0.0, m02: 0.0, m03: 0.0,
        m10: 0.0, m11: 1.0, m12: 0.0, m13: 0.0,
        m20: 0.0, m21: 0.0, m22: 1.0, m23: 0.0,
        m30: 0.0, m31: 0.0, m32: 0.0, m33: 1.0
    };

    fn new(c0: Vector4, c1: Vector4, c2: Vector4, c3: Vector4) -> Matrix4x4 {
        Matrix4x4 {
            m00: c0.x, m01: c1.x, m02: c2.x, m03: c3.x,
            m10: c0.y, m11: c1.y, m12: c2.y, m13: c3.y,
            m20: c0.z, m21: c1.z, m22: c2.z, m23: c3.z,
            m30: c0.w, m31: c1.w, m32: c2.w, m33: c3.w
        }
    }

    fn make_from_trs() -> Matrix4x4 {
        unimplemented!();
    }

    fn make_from_rotation(q: Quaternion) -> Matrix4x4 {
        unimplemented!();
    }
}

impl fmt::Debug for Matrix4x4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            "|{}, {}, {}, {}|
            |{}, {}, {}, {}|
            |{}, {}, {}, {}|
            |{}, {}, {}, {}|"
            m00, m01, m02, m03,
            m10, m11, m12, m13,
            m20, m21, m22, m23,
            m30, m31, m32, m33
        );
    }
}

impl fmt::Display for Matrix4x4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            "|{}, {}, {}, {}|
            |{}, {}, {}, {}|
            |{}, {}, {}, {}|
            |{}, {}, {}, {}|"
            m00, m01, m02, m03,
            m10, m11, m12, m13,
            m20, m21, m22, m23,
            m30, m31, m32, m33
        );
    }
}