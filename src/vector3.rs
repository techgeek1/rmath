use std::ops::*;
use std::f32::EPSILON;
use math::*;

#[derive(Clone, Copy, PartialEq)]
struct Vector3 {
    x: f32,
    y: f32,
    z: f32
}

#[allow(dead_code)]
impl Vector3 {
    const ZERO: Vector3 = Vector3{ x: 0.0, y: 0.0, z: 0.0 };
    const ONE: Vector3 = Vector3 { x: 1.0, y: 1.0, z: 1.0 };
    const FORWARD: Vector3 = Vector3 { x: 0.0, y: 0.0, z: 1.0 };
    const RIGHT: Vector3 = Vector3 { x: 1.0, y: 0.0, z: 0.0 };
    const UP: Vector3 = Vector3 { x: 0.0, y: 1.0, z: 0.0 };

    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 {
            x: x,
            y: y,
            z: z
        }
    }

    pub fn sqr_magnitude(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&mut self) {
        let mag = self.magnitude();
        if mag > EPSILON {
            *self = *self / mag;
        }
        else {
            *self = Vector3::ZERO;
        }
    }

    pub fn normalized(&self) -> Vector3 {
        let mag = self.magnitude();
        if mag > EPSILON {
            return *self / mag;
        }
        
        Vector3::ZERO
    }

    pub fn dot(&self, other: Vector3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }
    
    pub fn distance(&self, other: Vector3) -> f32 {
        (*self - other).magnitude()
    }

    pub fn angle(&self, other: Vector3) -> f32 {
        self.normalized()
            .dot(other.normalized())
            .clamp(-1.0, 1.0)
            .acos()
    }

    pub fn scale(&self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }

    pub fn clamp_magnitude(&self, max_length: f32) -> Vector3 {
        if self.sqr_magnitude() > max_length * max_length {
            return self.normalized() * max_length
        }
        
        *self
    }
    
    pub fn ortho_normalize(&mut self, other: &mut Vector3) {
        self.normalize();

        let mut c = self.cross(*other);
        c.normalize();

        *other = self.cross(*other);
        other.normalize();
    }

    
    pub fn lerp_to(&self, end: Vector3, t: f32) -> Vector3 {
        let alpha = t.clamp01();

        Vector3 {
            x: self.x + (end.x - self.x) * alpha,
            y: self.y + (end.y - self.y) * alpha,
            z: self.z + (end.z - self.z) * alpha
        }
    }

    pub fn lerp_to_unclamped(&self, end: Vector3, t: f32) -> Vector3 {
        Vector3 {
            x: self.x + (end.x - self.x) * t,
            y: self.y + (end.y - self.y) * t,
            z: self.z + (end.z - self.z) * t
        }
    }

    pub fn slerp_to(&self, end: Vector3, t: f32) -> Vector3 {
        let alpha = t.clamp01();
        let dot = self.dot(end)
            .clamp(-1.0, 1.0);
        
        let theta = dot.acos() * alpha;
        
        let mut relative = end - *self * dot;
        relative.normalize();

        (*self * theta.cos()) + (relative * theta.sin())
    }

    pub fn slerp_to_unclamped(&self, end: Vector3, t: f32) -> Vector3 {
        let dot = self.dot(end)
            .clamp(-1.0, 1.0);
        
        let theta = dot.acos() * t;
        
        let mut relative = end - *self * dot;
        relative.normalize();

        (*self * theta.cos()) + (relative * theta.sin())
    }

    pub fn project(&self, normal: Vector3) -> Vector3 {
        let dot = normal.dot(normal);
        if dot < EPSILON {
            Vector3::ZERO
        }
        else {
            *self * self.dot(normal) / dot
        }
    }
    
    pub fn project_on_segment(&self, start: Vector3, end: Vector3) -> Vector3 {
        let segment = end - start;
        let proj_point = self.project(segment.normalized());
        
        (proj_point - start).clamp_magnitude(segment.magnitude())
    }

    pub fn project_on_plane(&self, normal: Vector3) -> Vector3 {
        *self - self.project(normal)
    }

    pub fn reflect(&self, normal: Vector3) -> Vector3 {
        -2.0 * normal.dot(*self) * normal + *self
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

impl AddAssign<f32> for Vector3 {
    fn add_assign(&mut self, other: f32) {
        self.x += other;
        self.y += other;
        self.z += other;
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

impl Mul<Vector3> for f32 {
    type Output = Vector3;
    
    fn mul(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: other.x * self,
            y: other.y * self,
            z: other.z * self
        }
    }
}

impl MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
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

impl DivAssign<f32> for Vector3 {
    fn div_assign(&mut self, other: f32) {
        self.x = self.x / other;
        self.y = self.y / other;
        self.z = self.z / other;
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

impl ToString for Vector3 {
    fn to_string(&self) -> String {
        format!("({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Methods
    #[test]
    fn constructor() {
        let v = Vector3::new(1.0, 1.0, 1.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 1.0);
        assert_eq!(v.z, 1.0);
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
    fn normalized() {
        let v = Vector3::new(5.0, 0.0, 0.0).normalized();

        assert_eq!(v.x, 1.0);
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
    fn dot_product_fluent() {
        let left = Vector3::new(-1.0, 0.0, 0.0);
        let right = Vector3::new(1.0, 0.0, 0.0);
        let forward = Vector3::new(0.0, 0.0, 1.0);

        let dot_one = right.dot(right);
        assert_eq!(dot_one, 1.0);

        let dot_neg_one = right.dot(left);
        assert_eq!(dot_neg_one, -1.0);

        let dot_zero = right.dot(forward);
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
    fn cross_product_fluent() {
        let right = Vector3::new(1.0, 0.0, 0.0);
        let forward = Vector3::new(0.0, 0.0, 1.0);

        let up = forward.cross(right);
        assert_eq!(up.y, 1.0);
    }
    
    #[test]
    fn distance() {
        let v0 = Vector3::new(1.0, 0.0, 0.0);
        let v1 = Vector3::ZERO;
        
        let distance = Vector3::distance(v0, v1);
        
        assert_eq!(distance, 1.0);
    }
    
    #[test]
    fn distance_fluent() {
        let v0 = Vector3::new(1.0, 0.0, 0.0);
        let v1 = Vector3::ZERO;
        
        let distance = v0.distance(v1);
        
        assert_eq!(distance, 1.0);
    }
    
    #[test]
    fn angle() {
        let v0 = Vector3::new(1.0, 0.0, 0.0);
        let v1 = Vector3::new(0.0, 1.0, 0.0);
        
        let angle = Vector3::angle(v0, v1);
        
        assert_eq!(angle, 90.0.to_radians());
    }
    
    #[test]
    fn angle_fluent() {
        let v0 = Vector3::new(1.0, 0.0, 0.0);
        let v1 = Vector3::new(0.0, 1.0, 0.0);
        
        let angle = v0.angle(v1);
        
        assert_eq!(angle, 90.0.to_radians());
    }
    
    #[test]
    fn scale() {
        let v = Vector3::ONE;
        let v_scaled = Vector3::scale(v, Vector3::ONE * 4.0);
        
        assert_eq!(v_scaled.x, 4.0);
        assert_eq!(v_scaled.y, 4.0);
        assert_eq!(v_scaled.z, 4.0);
    }
    
    #[test]
    fn scale_fluent() {
        let v = Vector3::ONE;
        let v_scaled = v.scale(Vector3::ONE * 4.0);
        
        assert_eq!(v_scaled.x, 4.0);
        assert_eq!(v_scaled.y, 4.0);
        assert_eq!(v_scaled.z, 4.0);
    }
    
    #[test]
    fn clamp_magnitude() {
        let v = Vector3::ONE * 10.0;
        let v_clamped = Vector3::clamp_magnitude(v, 2.0);
        
        assert_eq!(v_clamped.magnitude(), 2.0);
    }
    
    #[test]
    fn clamp_magnitude_fluent() {
        let v = Vector3::ONE * 10.0;
        let v_clamped = v.clamp_magnitude(2.0);
        
        assert_eq!(v_clamped.magnitude(), 2.0);   
    }
    
    #[test]
    fn project() {
        assert!(false);
    }
    
    #[test]
    fn project_fluent() {
        assert!(false);
    }
    
    #[test]
    fn project_on_segment() {
        assert!(false);
    }
    
    #[test]
    fn project_on_segment_fluent() {
        assert!(false);
    }
    
    #[test]
    fn project_on_plane() {
        assert!(false);
    }
    
    #[test]
    fn project_on_plane_fluent() {
        assert!(false);
    }
    
    #[test]
    fn reflect() {
        assert!(false);
    }
    
    #[test]
    fn reflect_fluent() {
        assert!(false);
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