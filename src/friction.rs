use crate::Simulation;

/// a position with velocity that slows down due to drag.
///
/// This is good for objects that you fling&mdash;the scroll simulation uses this model in combination
/// with a spring. It can also be used in combination with a constant velocity for infinite
/// carousels, such as this one: <a href="https://cdn.rawgit.com/iamralpht/gravitas.js/master/examples/iTunesRadio/">Gravitas JavaScript Friction Example</a>.
#[derive(Copy, Clone)]
pub struct Friction {
    x: f32,
    v: f32,
    drag: f32,
    ln_drag: f32,
}
impl Friction {
    /// Create a new friction simulation with the given drag value. For scrolling interfaces where
    /// values are in pixels, a drag value of 0.001 feels quite good.
    pub fn new(drag: f32) -> Friction {
        Friction {
            x: 0.0,
            v: 0.0,
            drag,
            ln_drag: drag.ln(),
        }
    }
    /// Set the initial (time = 0.0) position and velocity for the friction simulation.
    pub fn set(&mut self, x: f32, v: f32) {
        self.x = x;
        self.v = v;
    }
    /// Return the time (in seconds) at which the friction simulation will reach the specified position. This
    /// value can be negative (which means the simulation would have reached that position if the velocity had
    /// been in the other direction) or not a number (NaN) which means the simulation will never reach that position.
    ///
    /// This method is used by the scroll simulation to find out the exact time that the scroll position will
    /// go beyond the scroll extent (at which time the velocity is put into a spring simulation which bounces the
    /// scroll position back to the extent).
    pub fn time_for_position(&self, p: f32) -> f32 {
        if (p - self.x).abs() < std::f32::EPSILON {
            0.0
        } else {
            (((p - self.x) * self.ln_drag + self.v) / self.v).ln() / self.ln_drag
        }
    }
}
impl Simulation for Friction {
    fn x(&self, time: f32) -> f32 {
        self.x + self.v * self.drag.powf(time) / self.ln_drag - self.v / self.ln_drag
    }
    fn dx(&self, time: f32) -> f32 {
        self.v * self.drag.powf(time)
    }
    fn is_done(&self, time: f32) -> bool {
        self.dx(time).abs() < 1.0
    }
}
