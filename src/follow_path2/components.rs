use crate::follow_path2::path::Path2;
use bevy::prelude::*;

/// Component that follows a 2D path at a specific speed
#[derive(Component)]
pub struct FollowPath2 {
    /// The [Path](Path2) to follow
    pub path: Path2,
    /// The index of the current point the entity is moving to
    pub cur_target: usize,
    /// Speed at which the entity moves
    pub speed: f32,
    /// Sets the epsilon for detecting when a point has been reached
    pub epsilon: f32,
}
