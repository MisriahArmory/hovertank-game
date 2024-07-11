use bevy::prelude::*;

use crate::{
    components::{ThirdPersonCamera, ThirdPersonCameraFocus},
    constants::camera::CAMERA_FOLLOW_SPEED,
};

pub fn camera(
    mut set: ParamSet<(
        Query<&mut Transform, With<ThirdPersonCamera>>,
        Query<&Transform, With<ThirdPersonCameraFocus>>,
    )>,
    time: Res<Time>,
) {
    let focus_transform = *set.p1().single();
    let focus_forward = focus_transform.forward();
    let focus_forward_xz = Vec3::new(focus_forward.x, 0.0, focus_forward.z).normalize();

    let camera_target_point = Vec3::new(
        focus_transform.translation.x,
        2.0,
        focus_transform.translation.z,
    ) - 4.0 * focus_forward_xz;
    let mut camera_transform_query = set.p0();
    let mut camera_transform = camera_transform_query.single_mut();
    let direction = camera_transform.translation - camera_target_point;

    if direction == Vec3::ZERO {
        return;
    }

    let dir_norm = direction.normalize();
    let dir_mag = direction.length();

    if dir_mag > 0.1 {
        camera_transform.translation -=
            dir_norm * dir_mag * CAMERA_FOLLOW_SPEED * time.delta_seconds();
    }
}
