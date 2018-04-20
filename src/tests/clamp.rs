use {Clamp, Clamp01, ApproxEq};

#[test]
fn clamp() {
    assert_approx_eq!(6.0.clamp(0.0, 1.0), 1.0);
}

#[test]
fn clamp01() {
    assert_approx_eq!(6.0.clamp01(), 1.0);
}