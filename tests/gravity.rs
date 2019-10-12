use gravitas::{Gravity, Simulation};

#[test]
fn test_gravity() {
    let g = Gravity::new();
    assert_eq!(g.x(0.0), 0.0);
    assert_eq!(g.dx(0.0), 0.0);
    assert!(!g.is_done(0.0));
    assert_eq!(g.x(1.0), 2450.0);
    let mut g = Gravity::new();
    g.set(10.0, -100.0);
    assert_eq!(g.x(0.0), 10.0);
    assert_eq!(g.dx(0.0), 0.0);
    assert_eq!(g.x(1.0), -40.0);
    assert_eq!(g.dx(1.0), -100.0);
    assert_eq!(g.dx(2.0), -200.0);
}
