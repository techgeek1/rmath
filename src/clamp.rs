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