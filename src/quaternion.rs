use std::ops::*;
use Vector3;

#[derive(Copy, Clone, PartialEq)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

#[allow(dead_code)]
impl Quaternion {
    /*
        Note: Use meh later https://www.wikiwand.com/en/Quaternions_and_spatial_rotation#/The_conjugation_operation
        https://www.3dgep.com/understanding-quaternions/#Adding_and_Subtracting_Quaternions
        http://www.euclideanspace.com/maths/geometry/rotations/conversions/quaternionToAngle/index.htm
    */
    
    pub const IDENTITY Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };
    
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
        unimplemented!();
    }

    pub fn from_euler(euler: Vector3) -> Quaternion{
        let e = euler * 0.5;
        
        let cos_x = e.x.cos();// pitch
        let sin_x = e.x.sin();
        let cos_y = e.y.cos();// yaw
        let sin_y = e.y.sin();
        let cos_z = e.z.cos();// roll
        let sin_z = e.z.sin();
        
        Quaternion {
            x: cos_y * sin_z * cos_x - sin_y * sin_z * sin_x,
            y: cos_y * cos_z * sin_x + sin_y * cos_z * sin_x,
            z: sin_y * cos_z * cos_x - sin_y * sin_z * cos_x,
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
        unimplemented!();
    }
    
    pub fn right(&self) -> Vector3 {
        unimplemented!();
    }
    
    pub fn up(&self) -> Vector3 {
        unimplemented!();
    }
    
    pub fn to_euler(&self) -> Vector3 {
        
    }

    pub fn to_angle_axis(&self, out_angle: &mut f32, out_axis: &mut Vector3) {
        if self.w > 1.0 {
            self.normalize();
        }
        
        out_angle = 2.0 * self.q.w.acos();
        let s = (1.0 - self.w * self.w).sqrt();
        if (s < 0.00001) {
            out_axis = Vector3 {
                x: self.x,
                y: self.y,
                z: self.z
            };
        }
        else {
            out_axis = Vector3 {
                x: self.x / s,
                y: self.y / s,
                x: self.z / s
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
            x: -self.x
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
        self.y = self.x / mag;
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

    pub fn slerp_to(&self, other: Quaternion, t: f32) -> Quaternion {
        unimplemented!();
    }
    
    pub fn squad_to(&self, other: Quaternion, t: f32) -> Quaternion {
        unimplemented!();
    }
}

impl PartialEq for Quaternion {
    fn eq(&self, other: &Quaternion) -> bool {
       dot(*self, *other) > 1.0 - std::f32::EPSILON; 
    }
}

impl Eq for Quaternion {}

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
        let x = self.x * 2.0;
        let y = self.y * 2.0;
        let z = self.z * 2.0;
        let xx = self.x * x;
        let yy = self.y * y;
        let zz = self.z * z;
        let xy = self.x * y;
        let xz = self.x * z;
        let yz = self.y * z;
        let wx = self.w * x;
        let wy = self.w * y;
        let wz = self.w * z;
        
        Vector3 {
            x: (1.0 - (yy + zz)) * other.x + (xy - wz) * other.y + (xz + wy) * other.z,
            y: (xy + wz) * other.x + (1.0 - (xx + zz)) * other.y + (yz - wx) * other.z,
            z: (xz - wy) * other.x + (yz + wx) * other.y + (1.0 - (xx + yy)) * other.z
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

mod test {
    use ApproxEq;
    use {Vector3, Quaternion};
    
    #[test]
    fn mul_quaternion_vector() {
        let v = Vector3::ONE;
        let q = Quaternion::new(0.0, 0.0, 0.0, 1.0);
        
        let v_rot = q * v;
        
        assert_approx_eq!(v, v_rot);
    }
}