use crate::constants::camera::{
    CAMERA_MAX_PITCH, CAMERA_MAX_PITCH_SPEED, CAMERA_MAX_YAW_SPEED, CAMERA_MIN_PITCH,
    CAMERA_PITCH_SPEED, CAMERA_YAW_SPEED,
};
use crate::traits::StableInterpolate;
use crate::{
    components::{LocalPlayer, ThirdPersonCamera},
    key_mappings::rotation::RotationAction,
    Rotor3,
};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub fn rotation_control(
    control_query: Query<&ActionState<RotationAction>, With<LocalPlayer>>,
    mut camera_query: Query<&mut Transform, With<ThirdPersonCamera>>,
    time: Res<Time>,
) {
    let action = control_query.single();
    let mut camera_transform = camera_query.single_mut();

    let Some(ax_data) = action.axis_pair(&RotationAction::Rotate) else {
        return;
    };

    let yaw_rotor = match ax_data.x() {
        0.0.. => Rotor3::from_rotation_arc(Vec3::X, Vec3::Z),
        ..0.0 => Rotor3::from_rotation_arc(Vec3::Z, Vec3::X),
        _ => Rotor3::IDENTITY,
    };

    let yaw_speed = (ax_data.x().abs() * CAMERA_YAW_SPEED).clamp(0.0, CAMERA_MAX_YAW_SPEED);
    let yaw_target = (yaw_rotor * camera_transform.rotation).normalize();
    camera_transform
        .rotation
        .smooth_nudge(&yaw_target, yaw_speed, time.delta_seconds());

    let forward = camera_transform.forward().into();
    let up = camera_transform.up().into();
    let pitch_rotor = match ax_data.y() {
        0.0.. => Rotor3::from_rotation_arc(up, forward),
        ..0.0 => Rotor3::from_rotation_arc(forward, up),
        _ => Rotor3::IDENTITY,
    };

    let pitch_target = (pitch_rotor * camera_transform.rotation).normalize();
    let pitch_speed = (ax_data.y().abs() * CAMERA_PITCH_SPEED).clamp(0.0, CAMERA_MAX_PITCH_SPEED);
    let current_rotation = camera_transform.rotation;
    camera_transform
        .rotation
        .smooth_nudge(&pitch_target, pitch_speed, time.delta_seconds());

    let forward = camera_transform.forward();
    let forward_xz = Vec3::new(forward.x, 0.0, forward.z);
    let pitch = forward.angle_between(forward_xz);

    if !(CAMERA_MIN_PITCH..=CAMERA_MAX_PITCH).contains(&pitch) {
        camera_transform.rotation = current_rotation;
    }
}
