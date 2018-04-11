use std::ops::*;
use std::f32::EPSILON;
use std::fmt;
use math::*;

// TODO:
// - Implement all reference variants for operators

#[derive(Clone, Copy)]
struct Vector3 {
    x: f32,
    y: f32,
    z: f32
}

impl Vector3 {
    #[allow(dead_code)] const ZERO: Vector3 = Vector3{ x: 0.0, y: 0.0, z: 0.0 };
    #[allow(dead_code)] const ONE: Vector3 = Vector3 { x: 1.0, y: 1.0, z: 1.0 };
    #[allow(dead_code)] const FORWARD: Vector3 = Vector3 { x: 0.0, y: 0.0, z: 1.0 };
    #[allow(dead_code)] const RIGHT: Vector3 = Vector3 { x: 1.0, y: 0.0, z: 0.0 };
    #[allow(dead_code)] const UP: Vector3 = Vector3 { x: 0.0, y: 1.0, z: 0.0 };

    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 {
            x: x,
            y: y,
            z: z
        }
    }
    
    pub fn clamp_magnitude(&self, max_length: f32) -> Vector3 {
        if self.sqr_magnitude() > max_length * max_length {
            return self.normalized() * max_length
        }
        
        *self
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

    pub fn dot(a: Vector3, b: Vector3) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn cross(a: Vector3, b: Vector3) -> Vector3 {
        Vector3 {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x
        }
    }
    
    pub fn distance(a: Vector3, b: Vector3) -> f32 {
        (a - b).magnitude()
    }

    pub fn angle(a: Vector3, b: Vector3) -> f32 {
        Vector3::dot(a.normalized(), b.normalized())
            .clamp(-1.0, 1.0)
            .acos()
    }

    pub fn scale(v: Vector3, other: Vector3) -> Vector3 {
        Vector3 {
            x: v.x * other.x,
            y: v.y * other.y,
            z: v.z * other.z
        }
    }
    
    pub fn ortho_normalize(a: &mut Vector3, b: &mut Vector3) {
        a.normalize();

        let mut c = Vector3::cross(*a, *b);
        c.normalize();

        *b = Vector3::cross(*a, *b);
        b.normalize();
    }

    
    pub fn lerp_to(start: Vector3, end: Vector3, t: f32) -> Vector3 {
        let alpha = t.clamp01();

        Vector3 {
            x: start.x + (end.x - start.x) * alpha,
            y: start.y + (end.y - start.y) * alpha,
            z: start.z + (end.z - start.z) * alpha
        }
    }

    pub fn lerp_to_unclamped(start: Vector3, end: Vector3, t: f32) -> Vector3 {
        Vector3 {
            x: start.x + (end.x - start.x) * t,
            y: start.y + (end.y - start.y) * t,
            z: start.z + (end.z - start.z) * t
        }
    }

    pub fn slerp_to(start: Vector3, end: Vector3, t: f32) -> Vector3 {
        let alpha = t.clamp01();
        let dot = Vector3::dot(start, end)
            .clamp(-1.0, 1.0);
        
        let theta = dot.acos() * alpha;
        
        let mut relative = end - start * dot;
        relative.normalize();

        (start * theta.cos()) + (relative * theta.sin())
    }

    pub fn slerp_to_unclamped(start: Vector3, end: Vector3, t: f32) -> Vector3 {
        let dot = Vector3::dot(start, end)
            .clamp(-1.0, 1.0);
        
        let theta = dot.acos() * t;
        
        let mut relative = end - start * dot;
        relative.normalize();

        (start * theta.cos()) + (relative * theta.sin())
    }

    pub fn project(v: Vector3, normal: Vector3) -> Vector3 {
        let dot = Vector3::dot(normal, normal);
        if dot < EPSILON {
            Vector3::ZERO
        }
        else {
            normal * Vector3::dot(v, normal) / dot
        }
    }
    
    pub fn project_on_segment(point: Vector3, start: Vector3, end: Vector3) -> Vector3 {
        let segment = end - start;
        let proj_point = Vector3::project(point, segment.normalized());
        
        (proj_point - start).clamp_magnitude(segment.magnitude())
    }

    pub fn project_on_plane(v: Vector3, normal: Vector3) -> Vector3 {
        v - Vector3::project(v, normal)
    }

    pub fn reflect(v: Vector3, normal: Vector3) -> Vector3 {
        -2.0 * Vector3::dot(normal, v) * normal + v
    }
}

// Formatting
impl ToString for Vector3 {
    fn to_string(&self) -> String {
        format!("({}, {}, {})", self.x, self.y, self.z)
    }
}

impl fmt::Debug for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

// Equality
impl PartialEq for Vector3 {
    fn eq(&self, other: &Vector3) -> bool {
        self.x.approx_eq(other.x) && self.y.approx_eq(other.y) && self.z.approx_eq(other.z)
    }
}

impl Eq for Vector3 {}

impl_op! { ApproxEq,
    fn approx_eq(self: Vector3, other: Vector3) -> bool {
        self.x.approx_eq(other.x) && self.y.approx_eq(other.y) && self.z.approx_eq(other.z)
    }
}

// Ops
impl_op! { Add,
    fn add(self: Vector3, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl_op! { Add,
    fn add(self: Vector3, other: f32) -> Vector3 {
        Vector3 {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other
        }
    }
}

impl_op! { Sub,
    fn sub(self: Vector3, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl_op! { Sub,
    fn sub(self: Vector3, other: f32) -> Vector3 {
        Vector3 {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other
        }
    }
}

impl_op! { Mul,
    fn mul(self: Vector3, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

impl_op! { Mul,
    fn mul(self: Vector3, other: f32) -> Vector3 {
        Vector3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        }
    }
}

impl_op! { Mul,
    fn mul(self: f32, other: Vector3) -> Vector3 {
        Vector3 {
            x: other.x * self,
            y: other.y * self,
            z: other.z * self
        }
    }
}

impl_op! { Div,
    fn div(self: Vector3, other: f32) -> Vector3 {
        Vector3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other
        }
    }
}

impl_op! { Neg,
    fn neg(self: Vector3) -> Vector3 {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl_op! { AddAssign,
    fn add_assign(&mut self: Vector3, other: f32) {
        self.x += other;
        self.y += other;
        self.z += other;
    }
}

impl_op! { MulAssign,
    fn mul_assign(&mut self: Vector3, other: f32) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl_op! { DivAssign,
    fn div_assign(&mut self: Vector3, other: f32) {
        self.x = self.x / other;
        self.y = self.y / other;
        self.z = self.z / other;   
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor() {
        let v = Vector3::new(1.0, 1.0, 1.0);
        assert_eq!(v, Vector3::ONE);
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
        assert_approx_eq!(dot_one, 1.0);

        let dot_neg_one = Vector3::dot(right, left);
        assert_approx_eq!(dot_neg_one, -1.0);

        let dot_zero = Vector3::dot(right, forward);
        assert_approx_eq!(dot_zero, 0.0);
    }
    
    #[test]
    fn cross_product() {
        let right = Vector3::new(1.0, 0.0, 0.0);
        let forward = Vector3::new(0.0, 0.0, 1.0);

        let up = Vector3::cross(forward, right);
        assert_eq!(up, Vector3::UP);
    }
    
    #[test]
    fn distance() {
        let v0 = Vector3::new(1.0, 0.0, 0.0);
        let v1 = Vector3::ZERO;
        
        let distance = Vector3::distance(v0, v1);
        
        assert_approx_eq!(distance, 1.0);
    }
    
    #[test]
    fn angle() {
        let v0 = Vector3::new(1.0, 0.0, 0.0);
        let v1 = Vector3::new(0.0, 1.0, 0.0);
        
        let angle = Vector3::angle(v0, v1);
        
        assert_approx_eq!(angle, 90.0_f32.to_radians());
    }

    #[test]
    fn scale() {
        let v = Vector3::ONE;
        let v_scaled = Vector3::scale(v, Vector3::ONE * 4.0);
        
        assert_eq!(v_scaled, Vector3::ONE * 4.0);
    }
    
    #[test]
    fn clamp_magnitude() {
        let v = Vector3::ONE * 10.0;
        let v_clamped = v.clamp_magnitude(2.0);
        
        assert_approx_eq!(v_clamped.magnitude(), 2.0);   
    }
    
    #[test]
    fn project() {
        let vector = Vector3::RIGHT * 2.0;
        let point = Vector3::RIGHT;
        
        let projected = Vector3::project(point, vector);
        
        assert_eq!(projected, Vector3::RIGHT);
    }
    

    #[test]
    fn project_on_segment() {
        let segment_end = Vector3::RIGHT * 2.0;
        let point = Vector3::RIGHT * 4.0;
        
        let projected = Vector3::project_on_segment(point, Vector3::ZERO, segment_end);
        
        assert_eq!(projected, Vector3::new(2.0, 0.0, 0.0));
    }
    
    #[test]
    fn project_on_plane() {
        let plane_normal = Vector3::FORWARD;
        let point = -Vector3::FORWARD * 4.0;
        
        let projected = Vector3::project_on_plane(point, plane_normal);
        
        assert_eq!(projected, Vector3::ZERO);
    }
    
    #[test]
    fn reflect() {
        let normal = Vector3::FORWARD;
        let vector = Vector3::new(-1.0, 0.0, -1.0);
        
        let reflected = Vector3::reflect(vector, normal);
        
        assert_eq!(reflected, Vector3::new(-1.0, 0.0, 1.0));
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
    fn mul_float_vector() {
        let v = 2.0 * Vector3::ONE;
        
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