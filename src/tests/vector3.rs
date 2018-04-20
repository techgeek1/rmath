use { Vector3, ApproxEq };

#[test]
fn constructor() {
    let v = Vector3::new(1.0, 1.0, 1.0);
    assert_eq!(v, Vector3::ONE);
}

#[test]
fn sqr_magnitude() {
    let mag = Vector3::new(5.0, 0.0, 0.0).sqr_magnitude();
    assert_eq!(mag, 5.0 * 5.0);
}

#[test]
fn magnitude() {
    let mag = Vector3::new(5.0, 0.0, 0.0).magnitude();
    assert_eq!(mag, 5.0);
}

#[test]
fn normalize_self() {
    let mut v = Vector3::new(5.0, 0.0, 0.0);
    v.normalize();

    assert_eq!(v.x, 1.0);
}

#[test]
fn normalized() {
    let v = Vector3::new(5.0, 0.0, 0.0).normalized();

    assert_eq!(v.x, 1.0);
}

#[test]
fn dot_product() {
    let left = Vector3::new(-1.0, 0.0, 0.0);
    let right = Vector3::new(1.0, 0.0, 0.0);
    let forward = Vector3::new(0.0, 0.0, 1.0);

    let dot_one = Vector3::dot(right, right);
    assert_eq!(dot_one, 1.0);

    let dot_neg_one = Vector3::dot(right, left);
    assert_eq!(dot_neg_one, -1.0);

    let dot_zero = Vector3::dot(right, forward);
    assert_eq!(dot_zero, 0.0);
}

#[test]
fn cross_product() {
    let right = Vector3::new(1.0, 0.0, 0.0);
    let forward = Vector3::new(0.0, 0.0, 1.0);

    let up = Vector3::cross(forward, right);
    assert_eq!(up, Vector3::UP);
}

#[test]
fn distance() {
    let v0 = Vector3::new(1.0, 0.0, 0.0);
    let v1 = Vector3::ZERO;

    let distance = Vector3::distance(v0, v1);

    assert_eq!(distance, 1.0);
}

#[test]
fn angle() {
    let v0 = Vector3::new(1.0, 0.0, 0.0);
    let v1 = Vector3::new(0.0, 1.0, 0.0);

    let angle = Vector3::angle(v0, v1);

    assert_eq!(angle, 90.0_f32.to_radians());
}

#[test]
fn scale() {
    let v = Vector3::ONE;
    let v_scaled = Vector3::scale(v, Vector3::ONE * 4.0);

    assert_eq!(v_scaled, Vector3::ONE * 4.0);
}

#[test]
fn clamp_magnitude() {
    let v = Vector3::ONE * 10.0;
    let v_clamped = v.clamp_magnitude(2.0);

    assert_approx_eq!(v_clamped.magnitude(), 2.0);   
}

#[test]
fn project() {
    let vector = Vector3::RIGHT * 2.0;
    let point = Vector3::RIGHT;

    let projected = Vector3::project(point, vector);

    assert_eq!(projected, Vector3::RIGHT);
}


#[test]
fn project_on_segment() {
    let segment_end = Vector3::RIGHT * 2.0;
    let point = Vector3::RIGHT * 4.0;

    let projected = Vector3::project_on_segment(point, Vector3::ZERO, segment_end);

    assert_eq!(projected, Vector3::new(2.0, 0.0, 0.0));
}

#[test]
fn project_on_plane() {
    let plane_normal = Vector3::FORWARD;
    let point = -Vector3::FORWARD * 4.0;

    let projected = Vector3::project_on_plane(point, plane_normal);

    assert_eq!(projected, Vector3::ZERO);
}

#[test]
fn reflect() {
    let normal = Vector3::FORWARD;
    let vector = Vector3::new(-1.0, 0.0, -1.0);

    let reflected = Vector3::reflect(vector, normal);

    assert_eq!(reflected, Vector3::new(-1.0, 0.0, 1.0));
}

#[test]
fn lerp() {
    let a = Vector3::ZERO;
    let b = Vector3::RIGHT;

    assert_eq!(Vector3::lerp(a, b, 0.0), Vector3::ZERO);
    assert_eq!(Vector3::lerp(a, b, 0.5), Vector3::new(0.5, 0.0, 0.0));
    assert_eq!(Vector3::lerp(a, b, 1.0), Vector3::RIGHT);
}

#[test]
fn lerp_unclamped() {
    let a = Vector3::ZERO;
    let b = Vector3::RIGHT;

    assert_eq!(Vector3::lerp_unclamped(a, b, -1.0), Vector3::new(-1.0, 0.0, 0.0));
    assert_eq!(Vector3::lerp_unclamped(a, b, 0.0), Vector3::ZERO);
    assert_eq!(Vector3::lerp_unclamped(a, b, 1.0), Vector3::RIGHT);
    assert_eq!(Vector3::lerp_unclamped(a, b, 2.0), Vector3::new(2.0, 0.0, 0.0));
}

// Operators
#[test]
fn add_scalar() {
    let v = Vector3::new(1.0, 0.0, 0.0) + 2.0;

    assert_eq!(v.x, 3.0);
    assert_eq!(v.y, 2.0);
    assert_eq!(v.z, 2.0);
}

#[test]
fn add_vector() {
    let a = Vector3::new(1.0, 0.0, 0.0);
    let b = Vector3::new(0.0, 0.0, 1.0);
    let v = a + b;

    assert_eq!(v.x, 1.0);
    assert_eq!(v.z, 1.0);
}

#[test]
fn sub_vector() {
    let a = Vector3::new(1.0, 0.0, 0.0);
    let b = Vector3::new(0.0, 0.0, 1.0);
    let v = b - a;

    assert_eq!(v.x, -1.0);
    assert_eq!(v.z, 1.0);
}

#[test]
fn mul_scalar() {
    let v = Vector3::new(1.0, 1.0, 1.0) * 2.0;

    assert_eq!(v.x, 2.0);
    assert_eq!(v.y, 2.0);
    assert_eq!(v.z, 2.0);
}

#[test]
fn mul_float_vector() {
    let v = 2.0 * Vector3::ONE;

    assert_eq!(v.x, 2.0);
    assert_eq!(v.y, 2.0);
    assert_eq!(v.z, 2.0);
}

#[test]
fn div_scalar() {
    let v = Vector3::new(2.0, 2.0, 2.0) / 2.0;

    assert_eq!(v.x, 1.0);
    assert_eq!(v.y, 1.0);
    assert_eq!(v.z, 1.0);
}

#[test]
fn neg_vector() {
    let v = Vector3::new(1.0, 1.0, 1.0);
    let neg_v = -v;

    assert_eq!(neg_v.x, -1.0);
    assert_eq!(neg_v.y, -1.0);
    assert_eq!(neg_v.z, -1.0);
}