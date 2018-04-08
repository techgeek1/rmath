use std;

pub trait Clamp {
    fn clamp(self, min: Self, max: Self) -> Self;
}

impl Clamp for f32 {
    fn clamp(self, min: f32, max: f32) -> f32 {
        if self < min {
            return min;
        } 

        if self > max {
            return max;
        }

        self
    }
}

pub trait Clamp01 {
    fn clamp01(self) -> Self;
}

impl Clamp01 for f32 {
    fn clamp01(self) -> f32 {
        if self < 0.0 {
            return 0.0;
        } 

        if self > 1.0 {
            return 1.0;
        }

        self
    }
}

pub trait ApproxEq<Other = Self> where Other: ?Sized {
    type Output;
    
    fn approx_eq(self, other: Other) -> bool;
}

macro_rules! impl_approx_eq {
    ($t: ty, $epsilon:expr) => {
        impl ApproxEq for $t {
            type Output = bool;
            
            fn approx_eq(self, other: $t) -> bool {
                if (self - other).abs() <= $epsilon {
                    return true;
                }

                if (self < 0.0) != (other < 0.0) {
                    return false;
                }

                let ulps_diff = ((self as i32) - (other as i32)).abs();
                if ulps_diff <= 1 {
                    return false;
                }

                false
            }
        }
        
        impl_ref_ops! { impl ApproxEq for $t, $t, approx_eq, bool }
    }
}

impl_approx_eq!(f32, std::f32::EPSILON);
impl_approx_eq!(f64, std::f64::EPSILON);

#[allow(unused_imports)]
mod tests {
    use math::*;
    
    #[test]
    fn clamp() {
        assert_approx_eq!(6.0.clamp(0.0, 1.0), 1.0);
    }
    
    #[test]
    fn clamp01() {
        assert_approx_eq!(6.0.clamp01(), 1.0);
    }
    
    #[test]
    fn approx_eq() {
        assert_approx_eq!(1.0_f32, 1.0_f32);
        assert_approx_eq!(1.0_f64, 1.0_f64);
    }
}
