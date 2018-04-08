use std::ops::*;

#[derive(Copy, Clone, PartialEq)]
struct Quaternion {
    x: f32,
    y: f32,
    z: f32,
    w: f32
}

#[allow(dead_code)]
impl Quaternion {
    /*
        Note: Use meh later https://www.wikiwand.com/en/Quaternions_and_spatial_rotation#/The_conjugation_operation
    */
    
    pub fn identity() -> Quaternion {
        Quaternion {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0
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

    pub fn from_direction(forward: Vector3) -> Quaternion {

    }

    pub fn from_orientation(forward: Vector3, up: Vector3) -> Quaternion {
        
    }

    pub fn from_euler(euler: Vector3) -> Quaternion{
        
    }

    pub fn from_angle_axis(angle: f32, axis: Vector3) -> Quaternion {

    }

    pub fn to_angle_axis(&self, &mut outAngle: f32, &mut outAxis: Vector3) {

    }
    
    pub fn to_euler(&self) -> Vector3 {

    }
    
    pub fn to_euler_rad(&self) -> Vevtor3 {
        
    }
    
    pub fn forward(&self) -> Vectro3 {
        
    }
    
    pub fn right(&self) -> Vector3 {
        unimplemented!();
    }
    
    pub fn up(&self) -> Vector3 {
        
    }

    pub fn dot(a: Quaternion, b: Quaternion) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
    }

    pub fn inverse(&self) -> Quaternion {
        let sqr_norm = self.sqr_norm();
        
        Quaternion {
            x: -self.x / sqr_norm,
            y: -self.y / sqr_norm,
            z: -self.z / sqr_norm,
            w: self.w / sqr_norm
        }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn sqr_magnitude(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }
    
    pub fn normalize(&mut self) {
        let mag = 1.0 / self.magnitude();
        
        self.x = self.x / mag;
        self.y = self.x / mag;
        self.z = self.z / mag;
        self.w = self.w / mag;
    }
    
    pub fn normalized(&self) -> Quaternion {
        
    }
    
    pub fn slerp_to(&self, other: Quaternion, t: f32) -> Quaternion {
        
    }
    
    pub fn squad_to(&self, other: Quaternion, t: f32) -> Quaternion {
        
    }
}

macro_rules! impl_add_Quaternion {
    () => {
        impl Add for Quaternion {
            type Output = Quaternion;

            fn add(self, other: Quaternion) -> Quaternion {
                Quaternion {
                    x: self.x + other.x,
                    y: self.y + other.y,
                    z: self.z + other.z,
                    w: self.w + other.w
                }
            }
        }
        
        impl_ref_ops! { impl Add for Quaternion, Quaternion, add, Quaternion }
    }
}

impl_add_Quaternion!();

macro_rules! impl_sub_Quaternion {
    () => {
        impl Sub for Quaternion {
            type Output = Quaternion;
            
            fn sub(self, other: Quaternion) -> Quaternion {
                Quaternion {
                    x: self.x - other.x,
                    y: self.y - other.y,
                    z: self.z - other.z,
                    w: self.w - other.w
                }
            }
        }
        
        impl_ref_ops! { impl Sub for Quaternion, Quaternion, sub, Quaternion }
    }
}

impl_sub_Quaternion!();

macro_rules! impl_mul_Quaternion {
    () => {
        impl Mul for Quaternion {
            type Output = Quaternion;

            fn mul(self, other: Quaternion) -> Quaternion {
                Quaternion {
                    x: self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y,
                    y: self.w * other.y - self.x * other.z + self.y * other.w + self.z * other.x,
                    z: self.w * other.z + self.x * other.y - self.y * other.x + self.z * other.w,
                    w: self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z
                }
            }
        }
        
        impl_ref_ops! { impl Mul for Quaternion, Quaternion, mul, Quaternion }
    }
}

impl_mul_Quaternion!();

macro_rules! impl_mul_Quaternion_Scalar {
    () => {
        impl Mul<f32> for Quaternion {
            type Output = Quaternion;
            
            fn mul(self, other: f32) -> Quaternion {
                Quaternion {
                    x: self.x * other,
                    y: self.y * other,
                    z: self.z * other,
                    w: self.w * other
                }
            }
        }
        
        impl_ref_ops! { impl Mul for Quaternion, f32, mul, Quaternion }
    }
}

impl_mul_Quaternion_Scalar!();

macro_rules! impl_mul_Quaternion_Vector {
    () => {
        impl Mul<Vector3> for Quaternion {
            type Output = Vector3;
            
            fn mul(self, other: Vector3) -> Vector3 {
                let p = Quaternion {
                    x: other.x,
                    y: other.y,
                    z: other.z,
                    w: 0.0
                };
                
                let p_prime = self * other * self.inverse();
                
                Vector3 {
                    x: p_prime.x,
                    y: p_prime.y,
                    z: p_prime.z
                }
            }
        }
        
        impl_ref_ops! { impl Mul for Quaternion, Vector3, mul, Quaternion }
    }
}

impl_mul_Quaternion_Vector!();

mod test {
    #[test]
    fn mul_quaternion_vector() {
        let v = Vector3::ONE;
        let q = Quaternion::new(0.0, 0.0, 0.0, 1.0);
        
        let v_rot = q * v;
        
        assert_approx_eq!(v, v_rot);
    }
}