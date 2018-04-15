use std::ops::{ Add, Sub, Mul, Div, Neg, AddAssign, SubAssign, MulAssign, DivAssign };
use std::cmp::{ PartialEq, Eq };
use std::fmt;

use {ApproxEq, Clamp01};
use consts::{ EPSILON };

#[derive(Clone, Copy)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl Vector4 {
    pub const ZERO: Vector4 = Vector4{ x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
    pub const ONE: Vector4 = Vector4 { x: 1.0, y: 1.0, z: 1.0, w: 1.0 };

    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vector4 {
        Vector4 {
            x: x,
            y: y,
            z: z,
            w: w
        }
    }
    
    pub fn clamp_magnitude(&self, max_length: f32) -> Vector4 {
        if self.sqr_magnitude() > max_length * max_length {
            return self.normalized() * max_length
        }
        
        *self
    }

    pub fn sqr_magnitude(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn normalize(&mut self) {
        let mag = self.magnitude();
        if mag > EPSILON {
            *self = *self / mag;
        }
        else {
            *self = Vector4::ZERO;
        }
    }

    pub fn normalized(&self) -> Vector4 {
        let mag = self.magnitude();
        if mag > EPSILON {
            return *self / mag;
        }
        
        Vector4::ZERO
    }

    pub fn dot(a: Vector4, b: Vector4) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
    }
    
    pub fn distance(a: Vector4, b: Vector4) -> f32 {
        (a - b).magnitude()
    }

    pub fn scale(v: Vector4, other: Vector4) -> Vector4 {
        Vector4 {
            x: v.x * other.x,
            y: v.y * other.y,
            z: v.z * other.z,
            w: v.w * other.w
        }
    }
    
    pub fn lerp(start: Vector4, end: Vector4, t: f32) -> Vector4 {
        let alpha = t.clamp01();

        Vector4 {
            x: start.x + (end.x - start.x) * alpha,
            y: start.y + (end.y - start.y) * alpha,
            z: start.z + (end.z - start.z) * alpha,
            w: start.w + (end.w - start.w) * alpha
        }
    }

    pub fn lerp_unclamped(start: Vector4, end: Vector4, t: f32) -> Vector4 {
        Vector4 {
            x: start.x + (end.x - start.x) * t,
            y: start.y + (end.y - start.y) * t,
            z: start.z + (end.z - start.z) * t,
            w: start.w + (end.w - start.w) * t
        }
    }

    pub fn project(v: Vector4, normal: Vector4) -> Vector4 {
        let dot = Vector4::dot(normal, normal);
        if dot < EPSILON {
            Vector4::ZERO
        }
        else {
            normal * Vector4::dot(v, normal) / dot
        }
    }
}

// Formatting
impl fmt::Debug for Vector4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

impl fmt::Display for Vector4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

// Equality
impl PartialEq for Vector4 {
    fn eq(&self, other: &Vector4) -> bool {
        self.x.approx_eq(other.x) && self.y.approx_eq(other.y) && self.z.approx_eq(other.z) && self.w.approx_eq(other.w)
    }
}

impl Eq for Vector4 {}

impl_op! { ApproxEq,
    fn approx_eq(self: Vector4, other: Vector4) -> bool {
        self.x.approx_eq(other.x) && self.y.approx_eq(other.y) && self.z.approx_eq(other.z) && self.w.approx_eq(other.w)
    }
}

// Ops
impl_op! { Add,
    fn add(self: Vector4, other: Vector4) -> Vector4 {
        Vector4 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w
        }
    }
}

impl_op! { Add,
    fn add(self: Vector4, other: f32) -> Vector4 {
        Vector4 {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
            w: self.w + other
        }
    }
}

impl_op! { Sub,
    fn sub(self: Vector4, other: Vector4) -> Vector4 {
        Vector4 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w
        }
    }
}

impl_op! { Sub,
    fn sub(self: Vector4, other: f32) -> Vector4 {
        Vector4 {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
            w: self.w - other
        }
    }
}

impl_op! { Mul,
    fn mul(self: Vector4, other: Vector4) -> Vector4 {
        Vector4 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w
        }
    }
}

impl_op! { Mul,
    fn mul(self: Vector4, other: f32) -> Vector4 {
        Vector4 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other
        }
    }
}

impl_op! { Mul,
    fn mul(self: f32, other: Vector4) -> Vector4 {
        Vector4 {
            x: other.x * self,
            y: other.y * self,
            z: other.z * self,
            w: other.w * self
        }
    }
}

impl_op! { Div,
    fn div(self: Vector4, other: f32) -> Vector4 {
        Vector4 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other
        }
    }
}

impl_op! { Neg,
    fn neg(self: Vector4) -> Vector4 {
        Vector4 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w
        }
    }
}

impl_op! { AddAssign,
    fn add_assign(&mut self: Vector4, other: f32) {
        self.x += other;
        self.y += other;
        self.z += other;
        self.w += other;
    }
}
    
impl_op! { SubAssign,
    fn sub_assign(&mut self: Vector4, other: f32) {
        self.x -= other;
        self.y -= other;
        self.z -= other;
        self.w -= other;
    }
}

impl_op! { MulAssign,
    fn mul_assign(&mut self: Vector4, other: f32) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
        self.w *= other;
    }
}

impl_op! { DivAssign,
    fn div_assign(&mut self: Vector4, other: f32) {
        self.x = self.x / other;
        self.y = self.y / other;
        self.z = self.z / other;
        self.w = self.w / other;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor() {
        let v = Vector4::new(1.0, 1.0, 1.0, 1.0);
        assert_eq!(v, Vector4::ONE);
    }

    #[test]
    fn sqr_magnitude() {
        let mag = Vector4::new(5.0, 0.0, 0.0, 0.0).sqr_magnitude();
        assert_eq!(mag, 5.0 * 5.0);
    }

    #[test]
    fn magnitude() {
        let mag = Vector4::new(5.0, 0.0, 0.0, 0.0).magnitude();
        assert_eq!(mag, 5.0);
    }

    #[test]
    fn normalize_self() {
        let mut v = Vector4::new(5.0, 0.0, 0.0, 0.0);
        v.normalize();

        assert_eq!(v.x, 1.0);
    }

    #[test]
    fn normalized() {
        let v = Vector4::new(5.0, 0.0, 0.0, 0.0).normalized();

        assert_eq!(v.x, 1.0);
    }
    
    #[test]
    fn dot_product() {
        let left = Vector4::new(-1.0, 0.0, 0.0, 0.0);
        let right = Vector4::new(1.0, 0.0, 0.0, 0.0);
        let forward = Vector4::new(0.0, 0.0, 1.0, 0.0);

        let dot_one = Vector4::dot(right, right);
        assert_approx_eq!(dot_one, 1.0);

        let dot_neg_one = Vector4::dot(right, left);
        assert_approx_eq!(dot_neg_one, -1.0);

        let dot_zero = Vector4::dot(right, forward);
        assert_approx_eq!(dot_zero, 0.0);
    }
    
    #[test]
    fn distance() {
        let v0 = Vector4::new(1.0, 0.0, 0.0, 0.0);
        let v1 = Vector4::ZERO;
        
        let distance = Vector4::distance(v0, v1);
        
        assert_approx_eq!(distance, 1.0);
    }

    #[test]
    fn scale() {
        let v = Vector4::ONE;
        let v_scaled = Vector4::scale(v, Vector4::ONE * 4.0);
        
        assert_eq!(v_scaled, Vector4::ONE * 4.0);
    }
    
    #[test]
    fn clamp_magnitude() {
        let v = Vector4::ONE * 10.0;
        let v_clamped = v.clamp_magnitude(2.0);
        
        assert_approx_eq!(v_clamped.magnitude(), 2.0);   
    }
    
    #[test]
    fn project() {
        let vector = Vector4::ONE * 2.0;
        let point = Vector4::ONE;
        
        let projected = Vector4::project(point, vector);
        
        assert_eq!(projected, Vector4::ONE);
    }
    
    #[test]
    fn lerp() {
        let a = Vector4::ZERO;
        let b = Vector4::ONE;
        
        assert_approx_eq!(Vector4::lerp(a, b, 0.0), Vector4::ZERO);
        assert_approx_eq!(Vector4::lerp(a, b, 0.5), Vector4::new(0.5, 0.5, 0.5, 0.5));
        assert_approx_eq!(Vector4::lerp(a, b, 1.0), Vector4::ONE);
    }
    
    #[test]
    fn lerp_unclamped() {
        let a = Vector4::ZERO;
        let b = Vector4::ONE;
        
        assert_approx_eq!(Vector4::lerp_unclamped(a, b, -1.0), Vector4::new(-1.0, -1.0, -1.0, -1.0));
        assert_approx_eq!(Vector4::lerp_unclamped(a, b, 0.0), Vector4::ZERO);
        assert_approx_eq!(Vector4::lerp_unclamped(a, b, 1.0), Vector4::ONE);
        assert_approx_eq!(Vector4::lerp_unclamped(a, b, 2.0), Vector4::new(2.0, 2.0, 2.0, 2.0));
    }
    
    // Operators
    #[test]
    fn add_scalar() {
        let v = Vector4::new(1.0, 0.0, 0.0, 0.0) + 2.0;

        assert_eq!(v.x, 3.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 2.0);
    }

    #[test]
    fn add_vector() {
        let a = Vector4::new(1.0, 0.0, 0.0, 0.0);
        let b = Vector4::new(0.0, 0.0, 1.0, 0.0);
        let v = a + b;

        assert_eq!(v.x, 1.0);
        assert_eq!(v.z, 1.0);
    }

    #[test]
    fn sub_vector() {
        let a = Vector4::new(1.0, 0.0, 0.0, 0.0);
        let b = Vector4::new(0.0, 0.0, 1.0, 0.0);
        let v = b - a;

        assert_eq!(v.x, -1.0);
        assert_eq!(v.z, 1.0);
    }

    #[test]
    fn mul_scalar() {
        let v = Vector4::new(1.0, 1.0, 1.0, 1.0) * 2.0;

        assert_eq!(v.x, 2.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 2.0);
        assert_eq!(v.w, 2.0);
    }
    
    #[test]
    fn mul_float_vector() {
        let v = 2.0 * Vector4::ONE;
        
        assert_eq!(v.x, 2.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 2.0);
        assert_eq!(v.w, 2.0);
    }

    #[test]
    fn div_scalar() {
        let v = Vector4::new(2.0, 2.0, 2.0, 2.0) / 2.0;

        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 1.0);
        assert_eq!(v.z, 1.0);
        assert_eq!(v.w, 1.0);
    }

    #[test]
    fn neg_vector() {
        let v = Vector4::new(1.0, 1.0, 1.0, 1.0);
        let neg_v = -v;

        assert_eq!(neg_v.x, -1.0);
        assert_eq!(neg_v.y, -1.0);
        assert_eq!(neg_v.z, -1.0);
        assert_eq!(neg_v.w, -1.0);
    }
}