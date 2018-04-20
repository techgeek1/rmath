use ApproxEq;
    
#[test]
fn approx_eq() {
    assert_approx_eq!(1.0_f32, 1.0_f32);
    assert_approx_eq!(1.0_f64, 1.0_f64);
}