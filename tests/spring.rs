use gravitas_rs::{Simulation, Spring};

#[test]
fn test_snapped() {
    let s = Spring::new(1.0, 400.0, 10.0);
    assert_eq!(s.x(0.0), 0.0);
    assert_eq!(s.dx(0.0), 0.0);
    assert!(s.is_done(0.0));
}
