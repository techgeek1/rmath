use std::ops::{ Add, Sub, Mul, AddAssign, SubAssign, MulAssign };
use std::cmp::{ PartialEq, Eq };
use std::fmt;

use {ApproxEq, Clamp01};
use consts::{ EPSILON, PI };
use Vector3;

const SIN_45: f32 = 0.8509035;
const COS_45: f32 = 0.5253219;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

#[allow(dead_code)]
impl Quaternion {
    /*
        Notes:
        https://www.wikiwand.com/en/Quaternions_and_spatial_rotation#/The_conjugation_operation
        https://www.3dgep.com/understanding-quaternions/#Adding_and_Subtracting_Quaternions
        http://www.euclideanspace.com/maths/geometry/rotations/conversions/quaternionToAngle/index.htm
        http://number-none.com/product/Understanding%20Slerp,%20Then%20Not%20Using%20It/
    */
    
    pub const IDENTITY : Quaternion = Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };
    
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Quaternion {
        Quaternion {
            x: x,
            y: y,
            z: z,
            w: w
        }
    }

    pub fn from_direction(forward: Vector3) -> Quaternion {
        Quaternion::from_orientation(forward, Vector3::UP)
    }

    pub fn from_orientation(forward: Vector3, up: Vector3) -> Quaternion {
        let forward = forward.normalized();
        let right = Vector3::cross(up, forward).normalized();
        let up = Vector3::cross(forward, right);
        
        let m00 = right.x;
        let m10 = right.y;
        let m20 = right.z;
        let m01 = up.x;
        let m11 = up.y;
        let m21 = up.z;
        let m02 = forward.x;
        let m12 = forward.y;
        let m22 = forward.z;
        
        let mut q = Quaternion {
            x: (1.0 + m00 - m11 - m22).max(0.0).sqrt() / 2.0,
            y: (1.0 - m00 + m11 - m22).max(0.0).sqrt() / 2.0,
            z: (1.0 - m00 - m11 + m22).max(0.0).sqrt() / 2.0,
            w: (1.0 + m00 + m11 + m22).max(0.0).sqrt() / 2.0 
        };

        q.x *= (m21 - m12).signum();
        q.y *= (m02 - m20).signum();
        q.z *= (m10 - m01).signum();

        q
    }
    
    pub fn from_euler(euler: Vector3) -> Quaternion{
        Quaternion::from_euler_components(euler.x, euler.y, euler.z)
    }
    
    pub fn from_euler_components(x: f32, y: f32, z: f32) -> Quaternion {
        let x = x / 2.0;
        let y = y / 2.0;
        let z = z / 2.0;
        
        // Pitch
        let sin_x = if x.abs().approx_eq(90.0) {
            SIN_45 * x.signum()
        }
        else {
            x.sin()
        };
        
        let cos_x = if x.abs().approx_eq(90.0) {
            COS_45 * x.signum()
        }
        else {
            x.cos()
        };
        
        // Yaw
        let sin_y = y.sin();
        let cos_y = y.cos();
        
        // Roll
        let sin_z = z.sin();
        let cos_z = z.cos();
        
        
        Quaternion {
            x: cos_y * sin_z * cos_x - sin_y * sin_z * sin_x,
            y: sin_y * cos_z * cos_x - sin_y * sin_z * cos_x,
            z: cos_y * cos_z * sin_x + sin_y * cos_z * sin_x,
            w: cos_y * cos_z * cos_x + cos_y * sin_z * sin_x
        }
    }

    pub fn from_angle_axis(angle: f32, axis: Vector3) -> Quaternion {
        let a = angle / 2.0;
        let sin_angle = a.sin();
        
        Quaternion {
            x: axis.x * sin_angle,
            y: axis.y * sin_angle,
            z: axis.z * sin_angle,
            w: a.cos()
        }
    }
    
    pub fn forward(&self) -> Vector3 {
        self * Vector3::FORWARD
    }
    
    pub fn right(&self) -> Vector3 {
        self * Vector3::RIGHT
    }
    
    pub fn up(&self) -> Vector3 {
        self * Vector3::UP
    }
    
    pub fn to_euler(&self) -> Vector3 {
        let x_sqr = self.x * self.x;
        let y_sqr = self.y * self.y;
        let z_sqr = self.z * self.z;
        let w_sqr = self.w * self.w;
        
        let unit = x_sqr + y_sqr + z_sqr + w_sqr;
        let test = self.x * self.y + self.z * self.w;
        if test > 0.5 * unit {
            return Vector3 {
                x: PI / 2.0,
                y: 2.0 * self.x.atan2(self.w),
                z: 0.0
            };
        }
        
        if test < -0.5 * unit {
            return Vector3 {
                x: -PI / 2.0,
                y: -2.0 * self.x.atan2(self.y),
                z: 0.0
            };
        }
        
        Vector3 {
            x: (2.0 * test / unit).asin(),
            y: (2.0 * self.y * self.w - 2.0 * self.x * self.z).atan2(x_sqr - y_sqr - z_sqr + w_sqr),
            z: (2.0 * self.x * self.w - 2.0 * self.y * self.z).atan2(-x_sqr + y_sqr - z_sqr + w_sqr)
        }
    }

    pub fn to_angle_axis(&self, out_angle: &mut f32, out_axis: &mut Vector3) {
        let q: Quaternion = if self.w > 1.0 {
            self.normalized()
        }
        else {
            *self
        };
        
        *out_angle = 2.0 * q.w.acos();
        let s = (1.0 - q.w * q.w).sqrt();
        if s < EPSILON {
            *out_axis = Vector3 {
                x: q.x,
                y: q.y,
                z: q.z
            };
        }
        else {
            *out_axis = Vector3 {
                x: q.x / s,
                y: q.y / s,
                z: q.z / s
            };
        }
    }
    
    pub fn dot(a: Quaternion, b: Quaternion) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
    }
    
    pub fn scale(q: Quaternion, scale: f32) -> Quaternion {
        Quaternion {
            x: q.x * scale,
            y: q.y * scale,
            z: q.z * scale,
            w: q.w * scale
        }
    }
    
    pub fn lerp(from: Quaternion, to: Quaternion, t: f32) -> Quaternion {
        Quaternion::lerp_unclamped(from, to, t.clamp01())
    }
    
    pub fn lerp_unclamped(from: Quaternion, to: Quaternion, t: f32) -> Quaternion {
        from * (1.0 - t) + to * t
    }
    
    pub fn slerp(from: Quaternion, to: Quaternion, t: f32) -> Quaternion {
        Quaternion::slerp_unclamped(from, to, t.clamp01())
    }
    
    pub fn slerp_unclamped(from: Quaternion, to: Quaternion, t: f32) -> Quaternion {
        let cos_half_theta = from.w * to.w + from.x * to.x + from.y * to.y + from.z * to.z;
        if cos_half_theta >= 1.0 {
            return from;
        }
        
        let b = if cos_half_theta < 0.0 {
            to.inverse()
        }
        else {
            to
        };
        
        let sin_half_theta = (1.0 - cos_half_theta * cos_half_theta).sqrt();
        if sin_half_theta.abs() < EPSILON {
            return Quaternion {
                x: from.x * 0.5 + b.x * 0.5,
                y: from.y * 0.5 + b.y * 0.5,
                z: from.z * 0.5 + b.z * 0.5,
                w: from.w * 0.5 + b.w * 0.5
            };
        }
        
        let half_theta = cos_half_theta.acos();
        let ratio_a = ((1.0 - t) * half_theta).sin() / sin_half_theta;
        let ratio_b = (t * half_theta).sin() / sin_half_theta;
    
        Quaternion {
            x: from.x * ratio_a + b.x * ratio_b,
            y: from.y * ratio_a + b.y * ratio_b,
            z: from.z * ratio_a + b.z * ratio_b,
            w: from.w * ratio_a + b.w * ratio_b
        }
    }

    pub fn inverse(&self) -> Quaternion {
        let sqr_norm = self.sqr_magnitude();
        
        Quaternion {
            x: -self.x / sqr_norm,
            y: -self.y / sqr_norm,
            z: -self.z / sqr_norm,
            w: self.w / sqr_norm
        }
    }
    
    pub fn conjugate(&self) -> Quaternion {
        Quaternion {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w
        }
    }
    
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn sqr_magnitude(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }
    
    pub fn normalize(&mut self) {
        let mag = self.magnitude();
        
        self.x = self.x / mag;
        self.y = self.y / mag;
        self.z = self.z / mag;
        self.w = self.w / mag;
    }
    
    pub fn normalized(&self) -> Quaternion {
        let mag = self.magnitude();
        
        Quaternion {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            w: self.w / mag
        }
    }
}

impl fmt::Debug for Quaternion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

impl fmt::Display for Quaternion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

impl PartialEq for Quaternion {
    fn eq(&self, other: &Quaternion) -> bool {
       Quaternion::dot(*self, *other) > (1.0 - EPSILON)
    }
}

impl Eq for Quaternion {}

impl_op! { ApproxEq,
    fn approx_eq(self: Quaternion, other: Quaternion) -> bool {
        Quaternion::dot(self, other) > (1.0 - EPSILON)
    }
}

impl_op! { Add,
    fn add(self: Quaternion, other: Quaternion) -> Quaternion {
        Quaternion {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w
        }
    }
}

impl_op! { Sub,
    fn sub(self: Quaternion, other: Quaternion) -> Quaternion {
        Quaternion {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w
        }
    }
}

impl_op! { Mul,
    fn mul(self: Quaternion, other: Quaternion) -> Quaternion {
        Quaternion {
            x: self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y,
            y: self.w * other.y - self.x * other.z + self.y * other.w + self.z * other.x,
            z: self.w * other.z + self.x * other.y - self.y * other.x + self.z * other.w,
            w: self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z
        }
    }
}

impl_op! { Mul,
    fn mul(self: Quaternion, other: Vector3) -> Vector3 {
        let x2 = self.x * 2.0;
        let y2 = self.y * 2.0;
        let z2 = self.z * 2.0;
        let w2 = self.w * 2.0;
        
        let xx = self.x * self.x;
        let yy = self.y * self.y;
        let zz = self.z * self.z;
        let ww = self.w * self.w;
        
        let xx_x = xx * other.x;
        let xx_y = xx * other.y;
        let xx_z = xx * other.z;
        
        let yy_x = yy * other.x;
        let yy_y = yy * other.y;
        let yy_z = yy * other.z;
        
        let zz_x = zz * other.x;
        let zz_y = zz * other.y;
        let zz_z = zz * other.z;
        
        let ww_x = ww * other.x;
        let ww_y = ww * other.y;
        let ww_z = ww * other.z;
        
        Vector3 {
            x: ww_x + (y2 * self.w * other.z) - (z2 * self.w * other.y) + xx_x + (y2 * self.x * other.y) + (z2 * self.x * other.z) - zz_x - yy_x,
            y: (x2 * self.y * other.x) + yy_y + (z2 * self.y * other.z) + (w2 * self.z * other.x) - zz_y + ww_y - (x2 * self.w * other.z) - xx_y,
            z: (x2 * self.z * other.x) + (y2 * self.z * other.y) + zz_z - (w2 * self.y * other.x) - yy_z + (w2 * self.x *other.y) - xx_z + ww_z
        }
    }
}

impl_op! { Mul,
    fn mul(self: Quaternion, other: f32) -> Quaternion {
        Quaternion {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other
        }
    }
}
    
impl_op! { AddAssign,
    fn add_assign(&mut self: Quaternion, other: Quaternion) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
        self.z = self.z + other.z;
        self.w = self.w + other.w;
    }    
}
    
impl_op! { SubAssign,
    fn sub_assign(&mut self: Quaternion, other: Quaternion) {
        self.x = self.x - other.x;
        self.y = self.y - other.y;
        self.z = self.z - other.z;
        self.w = self.w - other.w;
    }    
}
    
impl_op! { MulAssign,
    fn mul_assign(&mut self: Quaternion, other: Quaternion) {
        self.x = self.x * other.x;
        self.y = self.y * other.y;
        self.z = self.z * other.z;
        self.w = self.w * other.w;
    } 
}
    
impl_op! { MulAssign,
    fn mul_assign(&mut self: Quaternion, other: f32) {
        self.x = self.x * other;
        self.y = self.y * other;
        self.z = self.z * other;
        self.w = self.w * other;
    }    
}