use crate::{
    components::{ThirdPersonCamera, ThirdPersonCameraFocus},
    constants::camera::{CAMERA_FOLLOW_HEIGHT, CAMERA_FOLLOW_ROTATION_SPEED},
    traits::StableInterpolate,
    Rotor3,
};
use bevy::prelude::*;

pub fn camera_orbit_focus(
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

    {
        let relative_translation = focus_transform.translation - camera_transform.translation;
        let relative_translation_direction = relative_translation.normalize();
        let relative_translation_direction_xz = Vec3::new(
            relative_translation_direction.x,
            0.0,
            relative_translation_direction.z,
        )
        .normalize();

        let camera_forward_xz = Vec3::new(camera_forward.x, 0.0, camera_forward.z).normalize();
        let relative_angle = relative_translation_direction_xz.angle_between(camera_forward_xz);
        let orbit_speed = relative_angle * CAMERA_FOLLOW_ROTATION_SPEED;
        // We need to flip the arc here since we are usually behind the focus object and our rotation
        // is applied from the forward direction.
        let camera_target_rotor =
            Rotor3::from_rotation_arc(relative_translation_direction_xz, camera_forward_xz);
        let mut camera_orbit_rotor = Rotor3::IDENTITY;
        camera_orbit_rotor.smooth_nudge(&camera_target_rotor, orbit_speed, time.delta_seconds());

        let camera_orbit_translation = camera_orbit_rotor.mul_vec3(relative_translation);
        camera_transform.translation = focus_transform.translation - camera_orbit_translation;
    }

    let focus_offset = focus_transform.translation + Vec3::Y * CAMERA_FOLLOW_HEIGHT;
    let relative_translation = focus_offset - camera_transform.translation;
    let relative_translation_direction = relative_translation.normalize();

    let camera_forward = camera_transform.forward().into();
    let relative_angle = relative_translation_direction.angle_between(camera_forward);

    let orbit_speed = relative_angle * CAMERA_FOLLOW_ROTATION_SPEED;
    let camera_target_rotor =
        Rotor3::from_rotation_arc(relative_translation_direction, camera_forward);

    let mut camera_orbit_rotor = Rotor3::IDENTITY;
    camera_orbit_rotor.smooth_nudge(&camera_target_rotor, orbit_speed, time.delta_seconds());

    let camera_orbit_translation = camera_orbit_rotor.mul_vec3(relative_translation);
    camera_transform.translation = focus_offset - camera_orbit_translation;
}
