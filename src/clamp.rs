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

#[allow(unused_imports)]
mod tests {
    use approx_eq::ApproxEq;
    use super::{Clamp, Clamp01};
    
    #[test]
    fn clamp() {
        assert_approx_eq!(6.0.clamp(0.0, 1.0), 1.0);
    }
    
    #[test]
    fn clamp01() {
        assert_approx_eq!(6.0.clamp01(), 1.0);
    }
}