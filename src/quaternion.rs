use std::ops:*;

#[derive(Copy, Clone)]
struct Quaternion {
    x: f32,
    y: f32,
    z: f32,
    w: f32
}

impl Quaternion {
    pub fn identity() -> Quaternion {
        Quaternion {
            0.0,
            0.0,
            0.0,
            1.0
        }
    }

    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Quaternion {
        Quaternion {
            x: x,
            y: y,
            z: z,
            w: w
        }
    }

    pub fn new (v: Vector3, w: f32) -> Quaternion {
        Quaternion {
            x: v.x,
            y: v.y,
            z: v.z,
            w: w
        }
    }

    pub fn from_direction(forward: Vector3) -> Quaternion {

    }

    pub fn from_orientation(forward: Vector3, up: Vector3) -> Quaternion {
        
    }

    pub fn from_euler(euler: Vector3) -> Quaternion{
        
    }

    pub fn to_euler(&self) -> Quaternion {

    }

    pub fn from_angle_axis(angle: f32, axis: Vector3) -> Quaternion {

    }

    pub fn to_angle_axis(&self, &mut outAngle, &mut outAxis) {

    }

    pub fn dot(a: Quaternion, b: Quaternion) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
    }

    pub fn conjugate(&self) -> Quaternion {
        Quaternion {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w
        }
    }

    pub fn inverse(&self) -> Quaternion {
        self.conjugate() / self.sqr_norm();
    }

    pub fn norm(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn sqr_norm(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }
}

impl Mul for Quaternion {
    fn mul(self, other: Quaternion) -> Quaternion {
        Quaternion {
            x: self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y,
            y: self.w * other.y - self.x * other.z + self.y * other.w + self.z * other.x,
            z: self.w * other.z + self.x * other.y - self.y * other.x + self.z * other.w,
            w: self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z
        }
    }
}

impl Mul<Vector3> for Quaternion {
    fn mul(self, other: Vector3) -> Quaternion {
        let x = self.x * 2.0;
        let y = self.y * 2.0;
        let z = self.z * 2.0;

        
    }
}