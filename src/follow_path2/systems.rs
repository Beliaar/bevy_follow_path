use crate::follow_path2::components::FollowPath2;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use euclid::approxeq::ApproxEq;

pub fn advance_paths(
    mut query: Query<(Entity, &mut Transform, &mut FollowPath2)>,
    mut commands: Commands,
) {
    for (entity, mut transform, mut follow_path) in query.iter_mut() {
        let (distance, cur_target) = {
            let cur_target = follow_path.path.points[follow_path.cur_target];
            let distance = cur_target.distance(transform.translation.xy());
            if distance.approx_eq_eps(&0., &follow_path.epsilon) {
                let cur_target = follow_path.cur_target + 1;
                if cur_target < follow_path.path.points.len() {
                    follow_path.cur_target = cur_target;
                } else if follow_path.path.is_loop {
                    follow_path.cur_target = 0;
                } else {
                    commands.entity(entity).remove::<FollowPath2>();
                    return;
                }
                let cur_target = follow_path.path.points[follow_path.cur_target];
                (cur_target.distance(transform.translation.xy()), cur_target)
            } else {
                (distance, cur_target)
            }
        };
        let direction = (cur_target - transform.translation.xy()).normalize();

        let angle = direction.y.atan2(direction.x);

        transform.rotation = Quat::from_rotation_z(angle);

        let speed = follow_path.speed.min(distance);
        transform.translation.x += direction.x * speed;
        transform.translation.y += direction.y * speed;
    }
}
