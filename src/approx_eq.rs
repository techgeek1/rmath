use std;

pub trait ApproxEq<Other = Self> where Other: ?Sized {
    type Output;
    
    fn approx_eq(self, other: Other) -> bool;
}

macro_rules! impl_approx_eq {
    ($t: ty, $epsilon:expr) => {
        impl_op! { ApproxEq, 
            fn approx_eq(self: $t, other: $t) -> bool {
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
    }
}

impl_approx_eq!(f32, std::f32::EPSILON);
impl_approx_eq!(f64, std::f64::EPSILON);

#[allow(unused_imports)]
mod tests {
    use super::ApproxEq;
    
    #[test]
    fn approx_eq() {
        assert_approx_eq!(1.0_f32, 1.0_f32);
        assert_approx_eq!(1.0_f64, 1.0_f64);
    }
}
