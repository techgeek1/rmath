
mod math {
    pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
        if value < min {
            min
        } 
        
        if value > max {
            max
        }

        value
    }

    pub fn clamp01(value: f32) -> f32 {
        if value < 0.0 {
            0.0
        } 
        
        if value > 1.0 {
            1.0
        }

        value
    }
}