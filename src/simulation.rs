pub trait Simulation {
    fn x(&self, time: f32) -> f32;
    fn dx(&self, time: f32) -> f32;
    fn is_done(&self, time: f32) -> bool;
}
