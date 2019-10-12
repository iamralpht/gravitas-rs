use crate::{Friction, Simulation, Spring};

// A combination of friction and springs to create a touch-driven scrolling simulation.
#[derive(Clone, Copy)]
pub struct Scroll {
    extent: f32,
    friction: Friction,
    spring: Spring,
    spring_time: f32, // when we transition into using a spring
}
impl Scroll {
    pub fn new(extent: f32) -> Scroll {
        Scroll {
            extent,
            friction: Friction::new(0.01),
            spring: Spring::new(1.0, 90.0, 20.0),
            spring_time: std::f32::NAN,
        }
    }
    pub fn set(&mut self, x: f32, v: f32) {
        self.friction.set(x, v);
        // If we're already into overscroll on either end then just start out in the spring. If
        // friction with our velocity is going to take us out of overscroll then we don't bother
        // with the spring.
        let time_to_zero = self.friction.time_for_position(0.0);
        let time_to_extent = self.friction.time_for_position(-self.extent);
        if x > 0.0 && (!time_to_zero.is_finite() || time_to_zero < 0.0) {
            self.spring_time = 0.0;
            self.spring.snap(x);
            self.spring.set(0.0, v, 0.0);
        } else if x < -self.extent && (!time_to_extent.is_finite() || time_to_extent < 0.0) {
            self.spring_time = 0.0;
            self.spring.snap(x);
            self.spring.set(-self.extent, v, 0.0);
        } else {
            // Figure out which extent we're heading towards and then calculate the time
            // we'll transition into the spring.
            if v >= 0.0 {
                self.spring.snap(0.0);
                self.spring_time = time_to_zero;
                self.spring
                    .set(0.0, self.friction.dx(self.spring_time), self.spring_time);
            } else {
                self.spring.snap(-self.extent);
                self.spring_time = time_to_extent;
                self.spring.set(
                    -self.extent,
                    self.friction.dx(self.spring_time),
                    self.spring_time,
                );
            }
        }
    }
    pub fn extent(&self) -> f32 {
        self.extent
    }
    fn in_spring(&self, time: f32) -> bool {
        self.spring_time.is_finite() && time >= self.spring_time
    }
}
impl Simulation for Scroll {
    fn x(&self, time: f32) -> f32 {
        if self.in_spring(time) {
            self.spring.x(time)
        } else {
            self.friction.x(time)
        }
    }
    fn dx(&self, time: f32) -> f32 {
        if self.in_spring(time) {
            self.spring.dx(time)
        } else {
            self.friction.dx(time)
        }
    }
    fn is_done(&self, time: f32) -> bool {
        if self.in_spring(time) {
            self.spring.is_done(time)
        } else {
            self.friction.is_done(time)
        }
    }
}
