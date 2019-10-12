use gravitas::{Gravity, Simulation};

#[test]
fn test_gravity() {
    let g = Gravity::new(9.8 * 500.0);
    assert_eq!(g.x(0.0), 0.0);
    assert_eq!(g.dx(0.0), 0.0);
    assert!(!g.is_done(0.0));
    assert_eq!(g.x(1.0), 2450.0);
    let mut g = Gravity::new(9.8 * 500.0);
    g.set(10.0, -100.0);
    assert_eq!(g.x(0.0), 10.0);
    assert_eq!(g.dx(0.0), -100.0);
    assert_eq!(g.x(1.0), 2360.0);
    assert_eq!(g.dx(1.0), 4800.0);
    assert_eq!(g.dx(2.0), 9700.0);
}
