use { Vector4 , ApproxEq };

#[test]
fn constructor() {
    let v = Vector4::new(1.0, 1.0, 1.0, 1.0);
    assert_eq!(v, Vector4::ONE);
}

#[test]
fn sqr_magnitude() {
    let mag = Vector4::new(5.0, 0.0, 0.0, 0.0).sqr_magnitude();
    assert_eq!(mag, 5.0 * 5.0);
}

#[test]
fn magnitude() {
    let mag = Vector4::new(5.0, 0.0, 0.0, 0.0).magnitude();
    assert_eq!(mag, 5.0);
}

#[test]
fn normalize_self() {
    let mut v = Vector4::new(5.0, 0.0, 0.0, 0.0);
    v.normalize();

    assert_eq!(v.x, 1.0);
}

#[test]
fn normalized() {
    let v = Vector4::new(5.0, 0.0, 0.0, 0.0).normalized();

    assert_eq!(v.x, 1.0);
}

#[test]
fn dot_product() {
    let left = Vector4::new(-1.0, 0.0, 0.0, 0.0);
    let right = Vector4::new(1.0, 0.0, 0.0, 0.0);
    let forward = Vector4::new(0.0, 0.0, 1.0, 0.0);

    let dot_one = Vector4::dot(right, right);
    assert_approx_eq!(dot_one, 1.0);

    let dot_neg_one = Vector4::dot(right, left);
    assert_approx_eq!(dot_neg_one, -1.0);

    let dot_zero = Vector4::dot(right, forward);
    assert_approx_eq!(dot_zero, 0.0);
}

#[test]
fn distance() {
    let v0 = Vector4::new(1.0, 0.0, 0.0, 0.0);
    let v1 = Vector4::ZERO;

    let distance = Vector4::distance(v0, v1);

    assert_approx_eq!(distance, 1.0);
}

#[test]
fn scale() {
    let v = Vector4::ONE;
    let v_scaled = Vector4::scale(v, Vector4::ONE * 4.0);

    assert_eq!(v_scaled, Vector4::ONE * 4.0);
}

#[test]
fn clamp_magnitude() {
    let v = Vector4::ONE * 10.0;
    let v_clamped = v.clamp_magnitude(2.0);

    assert_approx_eq!(v_clamped.magnitude(), 2.0);   
}

#[test]
fn project() {
    let vector = Vector4::ONE * 2.0;
    let point = Vector4::ONE;

    let projected = Vector4::project(point, vector);

    assert_eq!(projected, Vector4::ONE);
}

#[test]
fn lerp() {
    let a = Vector4::ZERO;
    let b = Vector4::ONE;

    assert_approx_eq!(Vector4::lerp(a, b, 0.0), Vector4::ZERO);
    assert_approx_eq!(Vector4::lerp(a, b, 0.5), Vector4::new(0.5, 0.5, 0.5, 0.5));
    assert_approx_eq!(Vector4::lerp(a, b, 1.0), Vector4::ONE);
}

#[test]
fn lerp_unclamped() {
    let a = Vector4::ZERO;
    let b = Vector4::ONE;

    assert_approx_eq!(Vector4::lerp_unclamped(a, b, -1.0), Vector4::new(-1.0, -1.0, -1.0, -1.0));
    assert_approx_eq!(Vector4::lerp_unclamped(a, b, 0.0), Vector4::ZERO);
    assert_approx_eq!(Vector4::lerp_unclamped(a, b, 1.0), Vector4::ONE);
    assert_approx_eq!(Vector4::lerp_unclamped(a, b, 2.0), Vector4::new(2.0, 2.0, 2.0, 2.0));
}

// Operators
#[test]
fn add_scalar() {
    let v = Vector4::new(1.0, 0.0, 0.0, 0.0) + 2.0;

    assert_eq!(v.x, 3.0);
    assert_eq!(v.y, 2.0);
    assert_eq!(v.z, 2.0);
}

#[test]
fn add_vector() {
    let a = Vector4::new(1.0, 0.0, 0.0, 0.0);
    let b = Vector4::new(0.0, 0.0, 1.0, 0.0);
    let v = a + b;

    assert_eq!(v.x, 1.0);
    assert_eq!(v.z, 1.0);
}

#[test]
fn sub_vector() {
    let a = Vector4::new(1.0, 0.0, 0.0, 0.0);
    let b = Vector4::new(0.0, 0.0, 1.0, 0.0);
    let v = b - a;

    assert_eq!(v.x, -1.0);
    assert_eq!(v.z, 1.0);
}

#[test]
fn mul_scalar() {
    let v = Vector4::new(1.0, 1.0, 1.0, 1.0) * 2.0;

    assert_eq!(v.x, 2.0);
    assert_eq!(v.y, 2.0);
    assert_eq!(v.z, 2.0);
    assert_eq!(v.w, 2.0);
}

#[test]
fn mul_float_vector() {
    let v = 2.0 * Vector4::ONE;

    assert_eq!(v.x, 2.0);
    assert_eq!(v.y, 2.0);
    assert_eq!(v.z, 2.0);
    assert_eq!(v.w, 2.0);
}

#[test]
fn div_scalar() {
    let v = Vector4::new(2.0, 2.0, 2.0, 2.0) / 2.0;

    assert_eq!(v.x, 1.0);
    assert_eq!(v.y, 1.0);
    assert_eq!(v.z, 1.0);
    assert_eq!(v.w, 1.0);
}

#[test]
fn neg_vector() {
    let v = Vector4::new(1.0, 1.0, 1.0, 1.0);
    let neg_v = -v;

    assert_eq!(neg_v.x, -1.0);
    assert_eq!(neg_v.y, -1.0);
    assert_eq!(neg_v.z, -1.0);
    assert_eq!(neg_v.w, -1.0);
}