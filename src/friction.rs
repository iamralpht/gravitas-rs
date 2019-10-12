use crate::Simulation;

/// Implementation of a friction model. It slows down proportional to the amount of drag.
#[derive(Copy, Clone)]
pub struct Friction {
    x: f32,
    v: f32,
    drag: f32,
    ln_drag: f32,
}
impl Friction {
    pub fn new(drag: f32) -> Friction {
        Friction {
            x: 0.0,
            v: 0.0,
            drag,
            ln_drag: drag.ln(),
        }
    }
    pub fn set(&mut self, x: f32, v: f32) {
        self.x = x;
        self.v = v;
    }
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
