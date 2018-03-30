use std::ops::*;

#[derive(Copy, Clone)]
struct Vector3 {
    x: f32,
    y: f32,
    z: f32
}

#[allow(dead_code)]
impl Vector3 {
    pub fn zero() -> Vector3 {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }

    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 {
            x: x,
            y: y,
            z: z
        }
    }

    pub fn dot(lhs: Vector3, rhs: Vector3) -> f32 {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }

    pub fn cross(lhs: Vector3, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: lhs.y * rhs.z - lhs.z * rhs.y,
            y: lhs.z * rhs.x - lhs.x * rhs.z,
            z: lhs.x * rhs.y - lhs.y * rhs.x
        }
    }

    pub fn sqr_magnitude(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&mut self) {
        let norm: f32 = self.magnitude();
        self.x = self.x / norm;
        self.y = self.y / norm;
        self.z = self.z / norm;
    }

    pub fn normalized(&self) -> Vector3 {
        let norm: f32 = self.magnitude();
        Vector3 {
            x: self.x / norm,
            y: self.y / norm,
            z: self.z / norm
        }
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Add<f32> for Vector3 {
    type Output = Vector3;

    fn add(self, other: f32) -> Vector3 {
        Vector3 {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other
        }
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: f32) -> Vector3 {
        Vector3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        }
    }
}

impl Div<f32> for Vector3 {  
    type Output = Vector3;

    fn div(self, other: f32) -> Vector3 {
        Vector3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other
        }
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3 {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Vector methods
    #[test]
    fn zero() {
        let v = Vector3::zero();
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
        assert_eq!(v.z, 0.0);
    }

    #[test]
    fn constructor() {
        let v = Vector3::new(1.0, 1.0, 1.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 1.0);
        assert_eq!(v.z, 1.0);
    }

    #[test]
    fn dot_product() {
        let left = Vector3::new(-1.0, 0.0, 0.0);
        let right = Vector3::new(1.0, 0.0, 0.0);
        let forward = Vector3::new(0.0, 0.0, 1.0);

        let dot_one = Vector3::dot(right, right);
        assert_eq!(dot_one, 1.0);

        let dot_neg_one = Vector3::dot(right, left);
        assert_eq!(dot_neg_one, -1.0);

        let dot_zero = Vector3::dot(right, forward);
        assert_eq!(dot_zero, 0.0);
    }

    #[test]
    fn cross_product() {
        let right = Vector3::new(1.0, 0.0, 0.0);
        let forward = Vector3::new(0.0, 0.0, 1.0);

        let up = Vector3::cross(forward, right);
        assert_eq!(up.y, 1.0);
    }

    #[test]
    fn sqr_magnitude() {
        let mag = Vector3::new(5.0, 0.0, 0.0).sqr_magnitude();
        assert_eq!(mag, 5.0 * 5.0);
    }

    #[test]
    fn magnitude() {
        let mag = Vector3::new(5.0, 0.0, 0.0).magnitude();
        assert_eq!(mag, 5.0);
    }

    #[test]
    fn normalize_self() {
        let mut v = Vector3::new(5.0, 0.0, 0.0);
        v.normalize();

        assert_eq!(v.x, 1.0);
    }

    #[test]
    fn normalize() {
        let v = Vector3::new(5.0, 0.0, 0.0).normalized();

        assert_eq!(v.x, 1.0);
    }

    // Operators
    #[test]
    fn add_scalar() {
        let v = Vector3::new(1.0, 0.0, 0.0) + 2.0;

        assert_eq!(v.x, 3.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 2.0);
    }

    #[test]
    fn add_vector() {
        let a = Vector3::new(1.0, 0.0, 0.0);
        let b = Vector3::new(0.0, 0.0, 1.0);
        let v = a + b;

        assert_eq!(v.x, 1.0);
        assert_eq!(v.z, 1.0);
    }

    #[test]
    fn sub_vector() {
        let a = Vector3::new(1.0, 0.0, 0.0);
        let b = Vector3::new(0.0, 0.0, 1.0);
        let v = b - a;

        assert_eq!(v.x, -1.0);
        assert_eq!(v.z, 1.0);
    }

    #[test]
    fn mul_scalar() {
        let v = Vector3::new(1.0, 1.0, 1.0) * 2.0;

        assert_eq!(v.x, 2.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 2.0);
    }

    #[test]
    fn div_scalar() {
        let v = Vector3::new(2.0, 2.0, 2.0) / 2.0;

        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 1.0);
        assert_eq!(v.z, 1.0);
    }

    #[test]
    fn neg_vector() {
        let v = Vector3::new(1.0, 1.0, 1.0);
        let neg_v = -v;

        assert_eq!(neg_v.x, -1.0);
        assert_eq!(neg_v.y, -1.0);
        assert_eq!(neg_v.z, -1.0);
    }
}