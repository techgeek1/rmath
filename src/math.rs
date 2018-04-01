use std::f32::EPSILON;

#[allow(unused_macros)]
macro_rules! assert_approx_eq {
    ($value: expr, $expected: expr) => (
        if !$value.approx_eq($expected) {
            panic!("value: {}\nexpected: {}", $value, $expected);
        }
    )
}

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
    fn approx_eq(self, other: Other) -> bool;
}

impl ApproxEq for f32 {
    fn approx_eq(self, other: f32) -> bool {
        if (self - other).abs() <= EPSILON {
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

impl<'a> ApproxEq<&'a f32> for f32 {
    fn approx_eq(self, other: &'a f32) -> bool {
        if (self - *other).abs() <= EPSILON {
            return true;
        }
        
        if (self < 0.0) != (*other < 0.0) {
            return false;
        }
        
        let ulps_diff = (self - *other).abs() as i32;
        if ulps_diff <= 1 {
            return false;
        }
        
        false
    }
}

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
    fn approx_eq_val() {
        assert_approx_eq!(1.0, 1.0);
    }

    #[test]
    fn approx_eq_ref() {
        assert_approx_eq!(1.0, &1.0);
    }
}
