use bevy::prelude::*;

use crate::{
    components::{ThirdPersonCamera, ThirdPersonCameraFocus},
    constants::camera::{CAMERA_FOLLOW_DISTANCE, CAMERA_FOLLOW_HEIGHT, CAMERA_FOLLOW_SPEED},
    traits::StableInterpolate,
};

pub fn camera_follow_focus(
    mut set: ParamSet<(
        Query<&mut Transform, With<ThirdPersonCamera>>,
        Query<&Transform, With<ThirdPersonCameraFocus>>,
    )>,
    time: Res<Time>,
) {
    let focus_transform = *set.p1().single_mut();
    let mut camera_transform_query = set.p0();
    let mut camera_transform = camera_transform_query.single_mut();

    let focus_offset = focus_transform.translation + Vec3::Y * CAMERA_FOLLOW_HEIGHT;

    let relative_translation = focus_offset - camera_transform.translation;
    let relative_translation_direction = relative_translation.normalize();

    let camera_target_point =
        focus_offset - relative_translation_direction * CAMERA_FOLLOW_DISTANCE;

    let follow_speed = CAMERA_FOLLOW_SPEED * relative_translation.length() / CAMERA_FOLLOW_DISTANCE;

    camera_transform.translation.smooth_nudge(
        &camera_target_point,
        follow_speed,
        time.delta_seconds(),
    );
}
