use crate::{Friction, Simulation, Spring};
use core::cmp::Ordering;

/// A SnapPoint is either an end point or a point of attraction. Every pager needs at least two
/// snap points to define the extents
#[derive(Clone, Copy, Debug)]
pub struct SnapPoint {
    /// The location of the snap point.
    pub value: f32,
    /// Whether we should snap at this snap point. We will snap to one point or the other if
    /// both have this value set. Otherwise we allow free movement between a point with snap
    /// set to false and a point with snap set to true.
    pub snap: bool,
}

/// Any number can be either between two snap points, or beyond one of the extents.
pub enum SnapQuery {
    /// The queried value lies between these two snap points.
    Between(SnapPoint, SnapPoint),
    /// The queried value lies beyond this snap point.
    Beyond(SnapPoint),
}

/// Pager is similar to `Scroll`, except it contains user supplied snap points which the simulation will be attracted to.
/// These snap points are supplied to the constructor.
#[derive(Clone)]
pub struct Pager {
    snap_points: Vec<SnapPoint>,
    friction: Friction,
    spring: Spring,
    spring_time: f32, // when we transition into using a spring
}
impl Pager {
    /// Create a new scroll simulation which allows scrolls between 0 and the given extent.
    pub fn new(snap_points: &[SnapPoint]) -> Pager {
        let sort_predicate =
            |a: &SnapPoint, b: &SnapPoint| a.value.partial_cmp(&b.value).unwrap_or(Ordering::Equal);
        let mut snaps = snap_points.to_vec();
        snaps.sort_by(sort_predicate);

        Pager {
            snap_points: snaps,
            friction: Friction::new(0.01),
            spring: Spring::new(1.0, 90.0, 20.0),
            spring_time: std::f32::NAN,
        }
    }
    /// Start a gesture-based scroll from the scroll position `x` with velocity `v`.
    pub fn set(&mut self, x: f32, v: f32) {
        self.friction.set(x, v);
        // We need to find the snap points that we're between. If we're beyond an extent then we
        // will spring back to the extent. Otherwise we will either spring or snap depending on
        // the setup and our velocity.
        //
        // Note that we only consider the two points where we're starting. We could look at where
        // velocity would end us and then snap, or add a "mandatory" flag to SnapPoint like CSS
        // snap points have and examine all the points between where we are and where friction will
        // take us.
        let snap_query = self.query(x);
        match snap_query {
            SnapQuery::Beyond(SnapPoint { value, snap: false }) => {
                // If our velocity will take us beyond the snap point, then just use that to get back,
                // otherwise we need to spring.
                let time_to_extent = self.friction.time_for_position(value);
                if time_to_extent.is_finite() && time_to_extent > 0.0 {
                    // Yep, friction will bring us back in bounds.
                    self.spring_time = std::f32::NAN;
                } else {
                    // Oh, looks like we need to spring.
                    self.spring_time = 0.0;
                    self.spring.snap(x);
                    self.spring.set(value, v, 0.0);
                }
            }
            SnapQuery::Beyond(SnapPoint { value, snap: true }) => {
                // Don't use friction here, just bounce to the point.
                self.spring_time = 0.0;
                self.spring.snap(x);
                self.spring.set(value, v, 0.0);
            }
            SnapQuery::Between(
                SnapPoint {
                    value: a,
                    snap: true,
                },
                SnapPoint {
                    value: b,
                    snap: true,
                },
            ) => {
                // We're between two points that snap so we've got to pick one of them and then snap to it.
                let end_point = self.friction.x(10000.0);
                let a_dist = (a - end_point).abs();
                let b_dist = (b - end_point).abs();
                let snap_target = if a_dist < b_dist { a } else { b };
                self.spring_time = 0.0;
                self.spring.snap(x);
                self.spring.set(snap_target, v, 0.0);
            }
            SnapQuery::Between(SnapPoint { value: a, .. }, SnapPoint { value: b, .. }) => {
                // We're between two points, but both of them do not snap, so we're going to do a regular
                // scroll. So let friction do its thing until/unless we hit one of the snap points, in
                // which case do a bounce.
                let time_to_a = self.friction.time_for_position(a);
                let time_to_b = self.friction.time_for_position(b);
                if time_to_a.is_finite() && time_to_a > 0.0 {
                    self.spring_time = time_to_a;
                    self.spring.snap(a);
                    self.spring
                        .set(a, self.friction.dx(self.spring_time), self.spring_time);
                } else if time_to_b.is_finite() && time_to_b > 0.0 {
                    self.spring_time = time_to_b;
                    self.spring.snap(b);
                    self.spring
                        .set(b, self.friction.dx(self.spring_time), self.spring_time);
                } else {
                    self.spring_time = std::f32::NAN;
                }
            }
        }
    }

    /// Figure out which snap points the given position is between. This can be used by external callers
    /// to determine if they are in an "overdrag" case where they should damp movement or not.
    pub fn query(&self, x: f32) -> SnapQuery {
        let mut less_than: Option<SnapPoint> = None;
        let mut greater_than: Option<SnapPoint> = None;
        // This could be optimized since the snap points are sorted.
        self.snap_points.iter().for_each(|snap| {
            // Find the smallest value that's greater than "x".
            if snap.value > x {
                greater_than = match greater_than {
                    None => Some(*snap),
                    Some(s) => {
                        if s.value > snap.value {
                            Some(*snap)
                        } else {
                            greater_than
                        }
                    }
                };
                return;
            }
            // Find the largest value that's less than or equal to "x".
            less_than = match less_than {
                None => Some(*snap),
                Some(s) => {
                    if snap.value > s.value {
                        Some(*snap)
                    } else {
                        less_than
                    }
                }
            };
        });
        match (less_than, greater_than) {
            (Some(less), Some(more)) => SnapQuery::Between(less, more),
            (Some(extent), None) => SnapQuery::Beyond(extent),
            (None, Some(extent)) => SnapQuery::Beyond(extent),
            // This shouldn't happen because we should always have some snap points,
            // but if it does happen then invent an extent at zero that we can bounce
            // back to.
            (None, None) => SnapQuery::Beyond(SnapPoint {
                value: 0.0,
                snap: true,
            }),
        }
    }

    /// Jump to a position with an animation.
    pub fn jump_to(&mut self, position: f32, time: f32) {
        let x = self.x(time);
        let dx = self.dx(time);

        self.spring_time = 0.0;
        self.spring.snap(x);
        self.spring.set(position, dx, 0.0);
    }

    fn in_spring(&self, time: f32) -> bool {
        self.spring_time.is_finite() && time >= self.spring_time
    }
}
impl Simulation for Pager {
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
