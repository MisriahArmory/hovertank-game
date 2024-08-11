use crate::components::LastInput;
use crate::constants::camera::{
    CAMERA_COMFORT_ZONE_ADJUSTMENT_SPEED, CAMERA_COMFORT_ZONE_MAX_PITCH,
    CAMERA_COMFORT_ZONE_MIN_PITCH, CAMERA_COMFORT_ZONE_TIME, CAMERA_MAX_PITCH,
    CAMERA_MAX_PITCH_SPEED, CAMERA_MAX_YAW_SPEED, CAMERA_MIN_PITCH,
};
use crate::traits::StableInterpolate;
use crate::{components::LocalPlayer, key_mappings::rotation::RotationAction, Rotor3};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub fn third_person_rotation_control(
    mut control_query: Query<(&mut LastInput, &ActionState<RotationAction>), With<LocalPlayer>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
) {
    let (mut last_input, action) = control_query.single_mut();
    let mut camera_transform = camera_query.single_mut();

    let ax_data = action.clamped_axis_pair(&RotationAction::Rotate);
    if ax_data.x != 0.0 || ax_data.y != 0.0 {
        last_input.time = time.elapsed_seconds_f64();
    }

    let yaw_rotor = match ax_data.x {
        0.0.. => Rotor3::from_rotation_arc(Vec3::X, Vec3::Z),
        ..0.0 => Rotor3::from_rotation_arc(Vec3::Z, Vec3::X),
        _ => Rotor3::IDENTITY,
    };

    let yaw_speed = ax_data.x.abs() * CAMERA_MAX_YAW_SPEED;
    let yaw_target = (yaw_rotor * camera_transform.rotation).normalize();
    camera_transform
        .rotation
        .smooth_nudge(&yaw_target, yaw_speed, time.delta_seconds());

    let forward = camera_transform.forward().into();
    let up = camera_transform.up().into();
    let pitch_rotor = match ax_data.y {
        0.0.. => Rotor3::from_rotation_arc(up, forward),
        ..0.0 => Rotor3::from_rotation_arc(forward, up),
        _ => Rotor3::IDENTITY,
    };

    let mut y = ax_data.y;
    let pitch_target = (pitch_rotor * camera_transform.rotation).normalize();
    let pitch_speed = y.abs() * CAMERA_MAX_PITCH_SPEED;
    let current_rotation = camera_transform.rotation;
    camera_transform
        .rotation
        .smooth_nudge(&pitch_target, pitch_speed, time.delta_seconds());

    let forward = camera_transform.forward();
    let forward_xz = Vec3::new(forward.x, 0.0, forward.z);
    let pitch = forward.angle_between(forward_xz) * forward.y.signum();

    let time_since_last_input = time.elapsed_seconds_f64() - last_input.time;

    // Move camera back into the comfort zone if sufficient time has elapsed
    if time_since_last_input > CAMERA_COMFORT_ZONE_TIME
        && !(CAMERA_COMFORT_ZONE_MIN_PITCH..=CAMERA_COMFORT_ZONE_MAX_PITCH).contains(&pitch)
    {
        y = CAMERA_COMFORT_ZONE_ADJUSTMENT_SPEED * pitch.signum();
        let forward = camera_transform.forward().into();
        let pitch_rotor = match y {
            0.0.. => Rotor3::from_rotation_arc(up, forward),
            ..0.0 => Rotor3::from_rotation_arc(forward, up),
            _ => Rotor3::IDENTITY,
        };
        let pitch_target = (pitch_rotor * camera_transform.rotation).normalize();
        let pitch_speed = y.abs() * CAMERA_MAX_PITCH_SPEED;
        camera_transform
            .rotation
            .smooth_nudge(&pitch_target, pitch_speed, time.delta_seconds());
    }

    if !(CAMERA_MIN_PITCH..=CAMERA_MAX_PITCH).contains(&pitch) {
        camera_transform.rotation = current_rotation;
    }
}
