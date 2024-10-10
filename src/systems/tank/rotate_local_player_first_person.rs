use avian3d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::{
    components::{LastInput, LocalPlayer, RotationProperties},
    constants::camera::{
        CAMERA_COMFORT_ZONE_ADJUSTMENT_SPEED, CAMERA_COMFORT_ZONE_MAX_PITCH,
        CAMERA_COMFORT_ZONE_MIN_PITCH, CAMERA_COMFORT_ZONE_TIME, CAMERA_MAX_PITCH,
        CAMERA_MAX_PITCH_SPEED, CAMERA_MAX_YAW_SPEED, CAMERA_MIN_PITCH,
    },
    key_mappings::rotation::RotationAction,
    Rotor3,
};

pub fn rotate_local_player_first_person(
    mut control_query: Query<(&mut LastInput, &ActionState<RotationAction>), With<LocalPlayer>>,
    mut tank_query: Query<
        (
            &mut ExternalTorque,
            &Inertia,
            &Transform,
            &RotationProperties,
        ),
        With<LocalPlayer>,
    >,
    time: Res<Time>,
) {
    let (mut tank_torque, tank_inertia, tank_transform, tank_rot_props) = tank_query.single_mut();

    // Get last action
    let (mut last_input, action) = control_query.single_mut();
    let ax_data = action.clamped_axis_pair(&RotationAction::Rotate);
    if ax_data.x != 0.0 || ax_data.y != 0.0 {
        last_input.time = time.elapsed_seconds_f64();
    }
    let mut target_transform = *tank_transform;

    // Calculate where the tank should be looking
    // Yaw
    let yaw_rotor = Rotor3::from_axis_angle(Vec3::Y, -ax_data.x * CAMERA_MAX_YAW_SPEED);
    let yaw_target = (yaw_rotor * tank_transform.rotation).normalize();
    target_transform.rotation = yaw_target;

    let rotation_diff = target_transform.rotation * tank_transform.rotation.inverse();
    let (axis, angle) = rotation_diff.to_axis_angle();
    let torque = tank_inertia.mul_vec3(axis * angle) * tank_rot_props.rotation_strength;

    tank_torque.apply_torque(torque);
    target_transform = *tank_transform;

    // Pitch
    let right = tank_transform.right();
    let right_xz = Vec3::new(right.x, 0.0, right.z);
    let pitch_rotor = Rotor3::from_axis_angle(right_xz, -ax_data.y * CAMERA_MAX_PITCH_SPEED);

    let pitch_target = (pitch_rotor * target_transform.rotation).normalize();
    let current_rotation = target_transform.rotation;
    let current_forward = target_transform.forward();
    let current_forward_xz = Vec3::new(current_forward.x, 0.0, current_forward.z);
    let current_pitch =
        current_forward.angle_between(current_forward_xz) * current_forward.y.signum();
    target_transform.rotation = pitch_target;

    let forward = target_transform.forward();
    let forward_xz = Vec3::new(forward.x, 0.0, forward.z);
    let pitch = forward.angle_between(forward_xz) * forward.y.signum();

    let time_since_last_input = time.elapsed_seconds_f64() - last_input.time;

    // Move camera back into the comfort zone if sufficient time has elapsed
    if time_since_last_input > CAMERA_COMFORT_ZONE_TIME
        && !(CAMERA_COMFORT_ZONE_MIN_PITCH..=CAMERA_COMFORT_ZONE_MAX_PITCH).contains(&pitch)
    {
        let y = CAMERA_COMFORT_ZONE_ADJUSTMENT_SPEED * pitch.signum() * 2.0;
        let right = target_transform.right();
        let right_xz = Vec3::new(right.x, 0.0, right.z);
        let pitch_rotor = Rotor3::from_axis_angle(right_xz, -y);
        let pitch_target = (pitch_rotor * target_transform.rotation).normalize();
        target_transform.rotation = pitch_target;
    }

    if (current_pitch > CAMERA_MAX_PITCH && pitch > current_pitch)
        || (current_pitch < CAMERA_MIN_PITCH && pitch < current_pitch)
    {
        target_transform.rotation = current_rotation;
    }

    let rotation_diff = target_transform.rotation * tank_transform.rotation.inverse();
    let (axis, angle) = rotation_diff.to_axis_angle();
    let torque = tank_inertia.mul_vec3(axis * angle) * tank_rot_props.rotation_strength;

    tank_torque.apply_torque(torque);
}
