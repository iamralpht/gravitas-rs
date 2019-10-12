use gravitas::{Friction, Simulation};

#[test]
fn test_friction_initial() {
    let f = Friction::new(0.1);
    assert!(f.x(0.0) == 0.0);
    assert!(f.dx(0.0) == 0.0);
    assert!(f.is_done(0.0));
    assert!(f.time_for_position(0.0) == 0.0);
    assert!(f.time_for_position(1.0).is_nan());
}

#[test]
fn test_friction() {
    let mut f = Friction::new(0.1);
    f.set(0.0, 10.0);
    assert!(f.x(0.0) == 0.0);
    assert!(f.dx(0.0) == 10.0);
    assert!(!f.is_done(0.0));
    assert!(f.time_for_position(0.0) == 0.0);
    assert!(f.time_for_position(3.0) > 0.5 && f.time_for_position(3.0) < 0.51);
    assert!(f.time_for_position(10.0).is_nan());
    assert!(f.time_for_position(-0.1) < 0.0);
    assert!(!f.is_done(1.0));
    assert!(f.is_done(3.0));
    assert!(f.dx(3.0) < 0.1);
}
