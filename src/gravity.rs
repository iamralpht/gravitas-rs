use crate::Simulation;

/// a position under the influence of gravity (or any other constant acceleration), as defined by Newton's 2nd Law: `F = ma`.
///
/// Newton's 2nd law, `F = ma`, integrates to `x' = x + v * t + 0.5 * a * t * t`, which is what this simulation uses to compute a position.
///
/// This simulation is nice for objects that are falling, or have to overcome gravity in some way.
/// * gravity combined with a spring simulation to make bouncy dialog boxes: <a href="https://cdn.rawgit.com/iamralpht/gravitas.js/master/examples/FallingDialogs/index.html">Gravitas JavaScript bouncy dialogs</a>.
/// * gravity used to make a lock screen, which must be dragged upwards to unlock: <a href="https://cdn.rawgit.com/iamralpht/gravitas.js/master/examples/LockScreen/index.html">Gravitas JavaScript lock screen</a>.
#[derive(Clone, Copy)]
pub struct Gravity {
    x: f32,
    v: f32,
    a: f32,
    stop: f32, // In case the gravity runs away with something.
}
impl Gravity {
    /// Create a new gravity siulation with the given acceleration. A value
    /// of 500 \* 9.8 (so 500px corresponds to 1 meter) is normally a good
    /// starting point.
    pub fn new(a: f32) -> Gravity {
        Gravity {
            x: 0.0,
            v: 0.0,
            a,
            stop: 32000.0,
        }
    }
    /// Set the initial position and velocity (in pixels per second) of the gravity simulation.
    pub fn set(&mut self, x: f32, v: f32) {
        self.x = x;
        self.v = v;
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
