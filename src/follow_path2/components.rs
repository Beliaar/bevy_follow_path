use crate::follow_path2::path::Path2;
use bevy::prelude::*;

#[derive(Component)]
pub struct FollowPath2 {
    pub path: Path2,
    pub cur_target: usize,
    pub speed: f32,
}
