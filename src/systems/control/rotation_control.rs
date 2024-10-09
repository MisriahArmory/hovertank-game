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

    let yaw_rotor = Rotor3::from_axis_angle(Vec3::Y, -ax_data.x * CAMERA_MAX_YAW_SPEED);
    let yaw_target = (yaw_rotor * tag_transform.rotation).normalize();
    tag_transform
        .rotation
        .smooth_nudge(&yaw_target, 1.5, time.delta_seconds());

    let right = tag_transform.right();
    let right_xz = Vec3::new(right.x, 0.0, right.z);
    let pitch_rotor = Rotor3::from_axis_angle(right_xz, -ax_data.y * CAMERA_MAX_PITCH_SPEED);

    let pitch_target = (pitch_rotor * tag_transform.rotation).normalize();
    let current_rotation = tag_transform.rotation;
    let current_forward = tag_transform.forward();
    let current_forward_xz = Vec3::new(current_forward.x, 0.0, current_forward.z);
    let current_pitch =
        current_forward.angle_between(current_forward_xz) * current_forward.y.signum();

    tag_transform
        .rotation
        .smooth_nudge(&pitch_target, 1.5, time.delta_seconds());

    let forward = tag_transform.forward();
    let forward_xz = Vec3::new(forward.x, 0.0, forward.z);
    let pitch = forward.angle_between(forward_xz) * forward.y.signum();

    let time_since_last_input = time.elapsed_seconds_f64() - last_input.time;

    // Move camera back into the comfort zone if sufficient time has elapsed
    if time_since_last_input > CAMERA_COMFORT_ZONE_TIME
        && !(CAMERA_COMFORT_ZONE_MIN_PITCH..=CAMERA_COMFORT_ZONE_MAX_PITCH).contains(&pitch)
    {
        let y = CAMERA_COMFORT_ZONE_ADJUSTMENT_SPEED * pitch.signum() * CAMERA_MAX_PITCH_SPEED;
        let right = tag_transform.right();
        let right_xz = Vec3::new(right.x, 0.0, right.z);
        let pitch_rotor = Rotor3::from_axis_angle(right_xz, -y);
        let pitch_target = (pitch_rotor * tag_transform.rotation).normalize();
        tag_transform
            .rotation
            .smooth_nudge(&pitch_target, 1.0, time.delta_seconds());
    }

    if (pitch > CAMERA_MAX_PITCH && pitch > current_pitch)
        || (pitch < CAMERA_MIN_PITCH && pitch < current_pitch)
    {
        tag_transform.rotation = current_rotation;
    }
}
