use crate::Simulation;
use core::cmp::Ordering;

#[derive(PartialEq, Clone, Copy)]
enum SpringSolution {
    Overdamped { r1: f32, r2: f32, c1: f32, c2: f32 },
    CriticallyDamped { r: f32, c1: f32, c2: f32 },
    Underdamped { w: f32, r: f32, c1: f32, c2: f32 },
    Snapped,
}

const E: f32 = std::f32::consts::E;

impl SpringSolution {
    fn solve(
        damping: f32,
        mass: f32,
        spring_constant: f32,
        initial: f32,
        velocity: f32,
    ) -> SpringSolution {
        // Solve the quadratic equation
        let cmk = damping * damping - 4.0 * mass * spring_constant;
        match cmk.partial_cmp(&0.0) {
            Some(Ordering::Greater) => {
                // Overdamped
                let r1 = (-damping - cmk.sqrt()) / (2.0 * mass);
                let r2 = (-damping + cmk.sqrt()) / (2.0 * mass);
                let c2 = (velocity - r1 * initial) / (r2 - r1);
                let c1 = initial - c2;
                SpringSolution::Overdamped { r1, r2, c1, c2 }
            }
            Some(Ordering::Less) => {
                // Underdamped
                let w = (4.0 * mass * spring_constant - damping * damping).sqrt() / (2.0 * mass);
                let r = -(damping / 2.0 * mass);
                let c1 = initial;
                let c2 = (velocity - r * initial) / w;
                SpringSolution::Underdamped { w, r, c1, c2 }
            }
            _ => {
                // Equal, or close enough.
                // Critically damped.
                let r = -damping / (2.0 * mass);
                let c1 = initial;
                let c2 = velocity / (r * initial);
                SpringSolution::CriticallyDamped { r, c1, c2 }
            }
        }
    }
    fn x(&self, time: f32) -> f32 {
        match self {
            SpringSolution::Overdamped { r1, r2, c1, c2 } => {
                c1 * E.powf(r1 * time) + c2 * E.powf(r2 * time)
            }
            SpringSolution::CriticallyDamped { r, c1, c2 } => (c1 + c2 * time) * E.powf(r * time),
            SpringSolution::Underdamped { w, r, c1, c2 } => {
                E.powf(r * time) * (c1 * (w * time).cos() + c2 * (w * time).sin())
            }
            SpringSolution::Snapped => 0.0,
        }
    }
    fn dx(&self, time: f32) -> f32 {
        match self {
            SpringSolution::Overdamped { r1, r2, c1, c2 } => {
                c1 * r1 * E.powf(r1 * time) + c2 * r2 * E.powf(r2 * time)
            }
            SpringSolution::CriticallyDamped { r, c1, c2 } => {
                let pow = E.powf(r * time);
                r * (c1 + c2 * time) * pow + c2 * pow
            }
            SpringSolution::Underdamped { w, r, c1, c2 } => {
                let pow = E.powf(r * time);
                let cos = (w * time).cos();
                let sin = (w * time).sin();
                pow * (c2 * w * cos - c1 * w * sin) + r * pow * (c2 * sin + c1 * cos)
            }
            SpringSolution::Snapped => 0.0,
        }
    }
}

const EPSILON: f32 = 0.001;
fn almost_equal(a: f32, b: f32, epsilon: f32) -> bool {
    (a > (b - epsilon)) && (a < (b + epsilon))
}
fn almost_zero(a: f32, epsilon: f32) -> bool {
    almost_equal(a, 0.0, epsilon)
}

/// a position controlled by a spring as defined by Hooke's law, `F = -kx * cv`.
///
/// Depending on the values specified for the spring's mass, constant and damping, the
/// spring may be underdamped, critically damped or overdamped.
/// <a href="http://www.stewartcalculus.com/data/CALCULUS%20Concepts%20and%20Contexts/upfiles/3c3-AppsOf2ndOrders_Stu.pdf">This textbook</a>
/// provides a good overview of the spring model used by Gravitas.
///
/// A critically damped spring satisfies: `damping * damping - 4 * mass * spring_constant == 0`.
#[derive(Clone, Copy)]
pub struct Spring {
    mass: f32,
    spring_constant: f32,
    damping: f32,
    end: f32, // end position
    solution: SpringSolution,
    start_time: f32, // typically zero, but not if we were reconfigured while animating.
}
impl Spring {
    /// Create a new spring with the given mass, spring constant and damping values.
    ///
    /// The spring starts out "snapped" to 0.0.
    pub fn new(mass: f32, spring_constant: f32, damping: f32) -> Spring {
        Spring {
            mass,
            spring_constant,
            damping,
            end: 0.0,
            solution: SpringSolution::Snapped, // start out with a snapped spring.
            start_time: 0.0,
        }
    }
    /// Set the spring's endpoint to the given position and velocity. If time is non-zero
    /// then the velocity of the spring at that time (before these new values are applied)
    /// is also included.
    pub fn set(&mut self, x: f32, velocity: f32, time: f32) {
        // If this is a request to go where we're already going then ignore it.
        if almost_equal(x, self.end, EPSILON) && almost_zero(velocity, EPSILON) {
            return;
        }

        // If no time was given then don't use the last solution at all.
        let pos = if time <= 0.0 {
            self.end
        } else {
            self.end + self.solution.x(time - self.start_time)
        };
        let vel = if time <= 0.0 {
            velocity
        } else {
            velocity + self.solution.dx(time - self.start_time)
        };

        // If we're already at the requested position and there's no velocity then ignore too.
        if almost_zero(pos - x, EPSILON) && almost_zero(vel, EPSILON) {
            return;
        }
        self.solution =
            SpringSolution::solve(self.damping, self.mass, self.spring_constant, pos - x, vel);
        self.end = x;
        self.start_time = time;
    }
    /// "Snap" the spring and set the value. The spring simulation will return this value
    /// with no velocity for all time (or until set is called again) once snapped.
    pub fn snap(&mut self, x: f32) {
        self.end = x;
        self.start_time = 0.0;
        self.solution = SpringSolution::Snapped;
    }
}
impl Simulation for Spring {
    fn x(&self, time: f32) -> f32 {
        self.end + self.solution.x(time - self.start_time)
    }
    fn dx(&self, time: f32) -> f32 {
        self.solution.dx(time - self.start_time)
    }
    fn is_done(&self, time: f32) -> bool {
        almost_equal(self.x(time), self.end, EPSILON) && almost_zero(self.dx(time), EPSILON)
    }
}
