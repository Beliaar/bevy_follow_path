use crate::follow_path3::components::FollowPath3;
use bevy::prelude::*;
use euclid::approxeq::ApproxEq;

/// Moves entities along their set path
pub fn advance_paths(
    mut query: Query<(Entity, &mut Transform, &mut FollowPath3)>,
    mut commands: Commands,
) {
    for (entity, mut transform, mut follow_path) in query.iter_mut() {
        let (distance, cur_target) = {
            let cur_target = follow_path.path.points[follow_path.cur_target];
            let distance = cur_target.distance(transform.translation);
            if distance.approx_eq_eps(&0., &follow_path.epsilon) {
                let cur_target = follow_path.cur_target + 1;
                if cur_target < follow_path.path.points.len() {
                    follow_path.cur_target = cur_target;
                } else if follow_path.path.is_loop {
                    follow_path.cur_target = 0;
                } else {
                    commands.entity(entity).remove::<FollowPath3>();
                    return;
                }
                let cur_target = follow_path.path.points[follow_path.cur_target];
                (cur_target.distance(transform.translation), cur_target)
            } else {
                (distance, cur_target)
            }
        };
        let direction = (cur_target - transform.translation).normalize();

        transform.look_at(cur_target, follow_path.up_axis);

        let speed = follow_path.speed.min(distance);
        transform.translation.x += direction.x * speed;
        transform.translation.y += direction.y * speed;
        transform.translation.z += direction.z * speed;
    }
}
