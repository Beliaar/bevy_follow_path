use crate::follow_path2::systems::advance_paths;
use bevy::prelude::*;

/// Plugin for FollowPath2
pub struct FollowPath2Plugin;

impl Plugin for FollowPath2Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(advance_paths);
    }
}
