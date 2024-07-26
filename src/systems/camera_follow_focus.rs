use bevy::prelude::*;

use crate::{
    components::{ThirdPersonCamera, ThirdPersonCameraFocus},
    constants::camera::{
        CAMERA_FOLLOW_DISTANCE, CAMERA_FOLLOW_HEIGHT, CAMERA_FOLLOW_ROTATION_SPEED,
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
    let camera_forward = camera_transform.forward();

    let relative_position_direction_xz =
        (focus_transform.translation - camera_transform.translation).project_normalized(Vec3::Y);
    let camera_distance = (focus_transform.translation - camera_transform.translation).length();

    let camera_forward_xz = camera_forward.project_normalized(Vec3::Y);
    let camera_orbit_rotor =
        -Quat::from_rotation_arc(relative_position_direction_xz, camera_forward_xz);
    let new_relative_position = camera_orbit_rotor.mul_vec3(relative_position_direction_xz);
    let target_orbit_point = focus_transform.translation + Vec3::Y * CAMERA_FOLLOW_HEIGHT
        - new_relative_position * camera_distance;

    let new_translation = camera_transform.translation.lerp(
        target_orbit_point,
        CAMERA_FOLLOW_ROTATION_SPEED * time.delta_seconds(),
    );

    let new_relative_position_direction_xz =
        (focus_transform.translation - new_translation).project_normalized(Vec3::Y);

    let camera_target_point = Vec3::new(
        focus_transform.translation.x,
        CAMERA_FOLLOW_HEIGHT,
        focus_transform.translation.z,
    ) - new_relative_position_direction_xz * CAMERA_FOLLOW_DISTANCE;

    camera_transform.translation = new_translation.lerp(
        camera_target_point,
        CAMERA_FOLLOW_SPEED * time.delta_seconds(),
    );
}
