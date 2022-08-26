use bevy::prelude::*;
#[cfg(feature = "debug_draw")]
use bevy_prototype_lyon::prelude::*;

use crate::follow_path2::systems::advance_paths;

pub struct FollowPath2Plugin;

impl Plugin for FollowPath2Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(advance_paths);
        #[cfg(feature = "debug_draw")]
        app.add_plugin(ShapePlugin);
    }
}
