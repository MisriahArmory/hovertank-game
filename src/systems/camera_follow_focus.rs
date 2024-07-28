use bevy::prelude::*;

use crate::{
    components::{ThirdPersonCamera, ThirdPersonCameraFocus},
    constants::camera::{
        CAMERA_FOLLOW_DISTANCE, CAMERA_FOLLOW_DISTANCE_XZ, CAMERA_FOLLOW_HEIGHT,
        CAMERA_FOLLOW_SPEED,
    },
    traits::Project,
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

    let relative_translation_direction = focus_transform.translation - camera_transform.translation;
    let relative_translation_direction_xz =
        relative_translation_direction.project_normalized(Vec3::Y);

    let camera_target_point = Vec3::new(
        focus_transform.translation.x,
        focus_transform.translation.y + CAMERA_FOLLOW_HEIGHT,
        focus_transform.translation.z,
    ) - relative_translation_direction_xz * CAMERA_FOLLOW_DISTANCE_XZ;

    let follow_speed =
        CAMERA_FOLLOW_SPEED * relative_translation_direction.length() / *CAMERA_FOLLOW_DISTANCE;

    camera_transform.translation = camera_transform
        .translation
        .lerp(camera_target_point, follow_speed * time.delta_seconds());
}
