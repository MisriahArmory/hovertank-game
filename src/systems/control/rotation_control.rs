use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    components::{LastInput, LocalPlayer},
    constants::camera::{
        CAMERA_COMFORT_ZONE_ADJUSTMENT_SPEED, CAMERA_COMFORT_ZONE_MAX_PITCH,
        CAMERA_COMFORT_ZONE_MIN_PITCH, CAMERA_COMFORT_ZONE_TIME, CAMERA_MAX_PITCH,
        CAMERA_MAX_PITCH_SPEED, CAMERA_MAX_YAW_SPEED, CAMERA_MIN_PITCH,
    },
    key_mappings::rotation::RotationAction,
    traits::StableInterpolate,
    Rotor3,
};

/// Responds to inputs by rotating the object with the given tag.
pub fn rotation_control<Tag: Component>(
    mut control_query: Query<(&mut LastInput, &ActionState<RotationAction>), With<LocalPlayer>>,
    mut tag_transform_query: Query<&mut Transform, With<Tag>>,
    time: Res<Time>,
) {
    let (mut last_input, action) = control_query.single_mut();
    let mut tag_transform = tag_transform_query.single_mut();

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
    let yaw_target = (yaw_rotor * tag_transform.rotation).normalize();
    tag_transform
        .rotation
        .smooth_nudge(&yaw_target, yaw_speed, time.delta_seconds());

    let forward = tag_transform.forward().into();
    let up = tag_transform.up().into();
    let pitch_rotor = match ax_data.y {
        0.0.. => Rotor3::from_rotation_arc(up, forward),
        ..0.0 => Rotor3::from_rotation_arc(forward, up),
        _ => Rotor3::IDENTITY,
    };

    let mut y = ax_data.y;
    let pitch_target = (pitch_rotor * tag_transform.rotation).normalize();
    let pitch_speed = y.abs() * CAMERA_MAX_PITCH_SPEED;
    let current_rotation = tag_transform.rotation;
    tag_transform
        .rotation
        .smooth_nudge(&pitch_target, pitch_speed, time.delta_seconds());

    let forward = tag_transform.forward();
    let forward_xz = Vec3::new(forward.x, 0.0, forward.z);
    let pitch = forward.angle_between(forward_xz) * forward.y.signum();

    let time_since_last_input = time.elapsed_seconds_f64() - last_input.time;

    // Move camera back into the comfort zone if sufficient time has elapsed
    if time_since_last_input > CAMERA_COMFORT_ZONE_TIME
        && !(CAMERA_COMFORT_ZONE_MIN_PITCH..=CAMERA_COMFORT_ZONE_MAX_PITCH).contains(&pitch)
    {
        y = CAMERA_COMFORT_ZONE_ADJUSTMENT_SPEED * pitch.signum();
        let forward = tag_transform.forward().into();
        let pitch_rotor = match y {
            0.0.. => Rotor3::from_rotation_arc(up, forward),
            ..0.0 => Rotor3::from_rotation_arc(forward, up),
            _ => Rotor3::IDENTITY,
        };
        let pitch_target = (pitch_rotor * tag_transform.rotation).normalize();
        let pitch_speed = y.abs() * CAMERA_MAX_PITCH_SPEED;
        tag_transform
            .rotation
            .smooth_nudge(&pitch_target, pitch_speed, time.delta_seconds());
    }

    if !(CAMERA_MIN_PITCH..=CAMERA_MAX_PITCH).contains(&pitch) {
        tag_transform.rotation = current_rotation;
    }
}
