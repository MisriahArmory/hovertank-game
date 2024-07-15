use bevy::prelude::*;

use crate::{
    components::{ThirdPersonCamera, ThirdPersonCameraFocus},
    constants::camera::{
        CAMERA_FOLLOW_DISTANCE, CAMERA_FOLLOW_HEIGHT, CAMERA_FOLLOW_SPEED, CAMERA_TRANS_EPS,
    },
};

pub fn camera_follow_focus(
    mut set: ParamSet<(
        Query<&mut Transform, With<ThirdPersonCamera>>,
        Query<&Transform, With<ThirdPersonCameraFocus>>,
    )>,
    time: Res<Time>,
) {
    let focus_transform = *set.p1().single_mut();

    let focus_forward = focus_transform.forward();
    let focus_forward_xz = Vec3::new(focus_forward.x, 0.0, focus_forward.z);
    let focus_forward_xz = if focus_forward_xz.length_squared() > 0.0 {
        focus_forward.normalize()
    } else {
        -Vec3::Z
    };

    let camera_target_point = Vec3::new(
        focus_transform.translation.x,
        CAMERA_FOLLOW_HEIGHT,
        focus_transform.translation.z,
    ) - CAMERA_FOLLOW_DISTANCE * focus_forward_xz;
    let mut camera_transform_query = set.p0();
    let mut camera_transform = camera_transform_query.single_mut();
    let translation_direction = camera_target_point - camera_transform.translation;

    if translation_direction == Vec3::ZERO {
        return;
    }

    let trans_dir_norm = translation_direction.normalize();
    let trans_dir_mag = translation_direction.length();

    if trans_dir_mag > CAMERA_TRANS_EPS {
        camera_transform.translation +=
            trans_dir_norm * trans_dir_mag * CAMERA_FOLLOW_SPEED * time.delta_seconds();
    }
}
