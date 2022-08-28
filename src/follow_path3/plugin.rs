use crate::follow_path3::systems::advance_paths;
use bevy::prelude::*;

/// Plugin for FollowPath3
pub struct FollowPath3Plugin;

impl Plugin for FollowPath3Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(advance_paths);
    }
}
