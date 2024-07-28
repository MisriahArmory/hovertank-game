use crate::{
    components::{ThirdPersonCamera, ThirdPersonCameraFocus},
    constants::camera::{
        CAMERA_FOLLOW_HEIGHT, CAMERA_FOLLOW_PITCH_ROTATION_SPEED, CAMERA_FOLLOW_YAW_ROTATION_SPEED,
    },
    traits::Orbit,
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
    let focus_offset = focus_transform.translation + Vec3::Y * CAMERA_FOLLOW_HEIGHT;

    camera_transform.translation = camera_transform.translation.orbit(
        focus_offset,
        camera_forward.into(),
        Vec3::Y,
        CAMERA_FOLLOW_YAW_ROTATION_SPEED,
        time.delta_seconds(),
    );

    camera_transform.translation = camera_transform.translation.orbit(
        focus_offset,
        camera_forward.into(),
        camera_transform.right().into(),
        CAMERA_FOLLOW_PITCH_ROTATION_SPEED,
        time.delta_seconds(),
    );
}
