/// common methods implemented by every simulation allowing easy integration into an animation system.
pub trait Simulation {
    /// Return the position for the given time (in seconds).
    fn x(&self, time: f32) -> f32;
    /// Return the velocity for the given time (in seconds).
    fn dx(&self, time: f32) -> f32;
    /// Return true if the simulation has reached a final position at the given time (in seconds).
    fn is_done(&self, time: f32) -> bool;
}
