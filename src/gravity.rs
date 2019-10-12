use crate::Simulation;

/// Gravity simulation, really Newton's 2nd law, F=ma, integrated to x' = x + v*t + 0.5*a*t*t.
#[derive(Clone, Copy)]
pub struct Gravity {
    x: f32,
    v: f32,
    a: f32,
    stop: f32, // In case the gravity runs away with something.
}
impl Default for Gravity {
    fn default() -> Gravity {
        Gravity {
            x: 0.0,
            v: 0.0,
            a: 9.8 * 500.0, // say 500px = 1m.
            stop: 32000.0,
        }
    }
}
impl Gravity {
    pub fn new() -> Gravity {
        Default::default()
    }
    pub fn set(&mut self, x: f32, a: f32) {
        self.x = x;
        self.a = a;
    }
}
impl Simulation for Gravity {
    fn x(&self, time: f32) -> f32 {
        self.x + self.v * time + 0.5 * self.a * time * time
    }
    fn dx(&self, time: f32) -> f32 {
        self.v + self.a * time
    }
    fn is_done(&self, time: f32) -> bool {
        self.x(time).abs() >= self.stop
    }
}
