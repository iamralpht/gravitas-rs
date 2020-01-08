//! Gravitas is a collection of equations for making UI elements move in response to touch gestures.
//!
//! There are also implementations for <a href="https://github.com/iamralpht/gravitas">Java</a> and <a href="https://github.com/iamralpht/gravitas.js">JavaScript</a> (which has some interactive examples).
//!
//! Each simulation models a single value and generally has a setup function that takes the initial position and velocity.
//! Normally you would compute these in response to a touch gesture ending. All of the simulations are parametric over
//! time and have been algebraically integrated (rather than using a numerical integration method at runtime). The advantage
//! of algebraic integration is lower CPU overhead, and no odd behavior if frames are dropped.
mod friction;
mod gravity;
mod pager;
mod scroll;
mod simulation;
mod spring;

pub use friction::Friction;
pub use gravity::Gravity;
pub use pager::{Pager, SnapPoint as PagerSnapPoint, SnapQuery as PagerSnapQuery};
pub use scroll::Scroll;
pub use simulation::Simulation;
pub use spring::Spring;
