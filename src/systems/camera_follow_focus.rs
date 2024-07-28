use bevy::prelude::*;

use crate::{
    components::{ThirdPersonCamera, ThirdPersonCameraFocus},
    constants::camera::{
        CAMERA_FOLLOW_DISTANCE, CAMERA_FOLLOW_HEIGHT, CAMERA_FOLLOW_ROTATION_SPEED,
        CAMERA_FOLLOW_SPEED,
    },
    traits::Project,
    Rotor3,
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

    let relative_position_direction = focus_transform.translation - camera_transform.translation;
    let relative_position_direction_xz = relative_position_direction.project_normalized(Vec3::Y);

    let camera_forward_xz = camera_forward.project_normalized(Vec3::Y);
    // We need to flip the arc here since we are usually behind the focus object and our rotation
    // is applied from the forward direction.
    let camera_target_rotor =
        Rotor3::from_rotation_arc(relative_position_direction_xz, camera_forward_xz);
    let camera_orbit_rotor = Rotor3::IDENTITY.slerp(
        camera_target_rotor,
        time.delta_seconds() * CAMERA_FOLLOW_ROTATION_SPEED,
    );

    let new_translation =
        focus_transform.translation - camera_orbit_rotor.mul_vec3(relative_position_direction);
    let new_relative_translation = focus_transform.translation - new_translation;

    let new_relative_translation_direction_xz =
        new_relative_translation.project_normalized(Vec3::Y);

    let camera_target_point = Vec3::new(
        focus_transform.translation.x,
        CAMERA_FOLLOW_HEIGHT,
        focus_transform.translation.z,
    ) - new_relative_translation_direction_xz * CAMERA_FOLLOW_DISTANCE;

    camera_transform.translation = new_translation.lerp(
        camera_target_point,
        CAMERA_FOLLOW_SPEED * time.delta_seconds(),
    );
}
