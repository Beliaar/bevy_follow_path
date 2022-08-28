use crate::follow_path3::path::Path3;
use bevy::prelude::*;

/// Component that follows a 3D path at a specific speed
#[derive(Component)]
pub struct FollowPath3 {
    /// The [Path](Path3) to follow
    pub path: Path3,
    /// The index of the current point the entity is moving to
    pub cur_target: usize,
    /// Speed at which the entity moves
    pub speed: f32,
    /// Sets the epsilon for detecting when a point has been reached
    pub epsilon: f32,
    /// Upwards pointing axis of the entity
    pub up_axis: Vec3,
}
