use crate::constants::camera::{
    CAMERA_MAX_PITCH, CAMERA_MIN_PITCH, CAMERA_PITCH_SPEED, CAMERA_YAW_SPEED,
};

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    components::{LocalPlayer, ThirdPersonCamera},
    key_mappings::rotation::RotationAction,
};

pub fn rotation_control(
    control_query: Query<&ActionState<RotationAction>, With<LocalPlayer>>,
    mut camera_query: Query<&mut Transform, With<ThirdPersonCamera>>,
    time: Res<Time>,
) {
    let action = control_query.single();
    let mut camera_transform = camera_query.single_mut();

    if let Some(ax_data) = action.axis_pair(&RotationAction::Rotate) {
        let yaw_rotation = match ax_data.x() {
            f32::NEG_INFINITY => Quat::from_axis_angle(Vec3::Y, 1.0),
            f32::INFINITY => Quat::from_axis_angle(Vec3::Y, -1.0),
            _ => Quat::IDENTITY,
        };

        camera_transform.rotation = camera_transform.rotation.slerp(
            yaw_rotation * camera_transform.rotation,
            CAMERA_YAW_SPEED * time.delta_seconds(),
        );
        camera_transform.rotation = camera_transform.rotation.normalize();

        let right = camera_transform.right().normalize();
        // project the "right" vector onto the xz plane to get our pitch axis
        let pitch_axis = (right - right.dot(Vec3::Y) / Vec3::Y.dot(Vec3::Y) * Vec3::Y).normalize();
        let pitch_rotation = match ax_data.y() {
            f32::NEG_INFINITY => Quat::from_axis_angle(pitch_axis, 1.0),
            f32::INFINITY => Quat::from_axis_angle(pitch_axis, -1.0),
            _ => Quat::IDENTITY,
        };

        let pitch_target = pitch_rotation * camera_transform.rotation;
        let new_rotation = camera_transform
            .rotation
            .slerp(pitch_target, CAMERA_PITCH_SPEED * time.delta_seconds());
        let mut rotation_target = camera_transform.with_rotation(new_rotation);

        let forward = rotation_target.forward().normalize();
        let forward_xz = Vec3::new(forward.x, 0.0, forward.z).normalize();
        let pitch = forward.dot(forward_xz).clamp(-1.0, 1.0).acos();

        if !(CAMERA_MIN_PITCH..CAMERA_MAX_PITCH).contains(&pitch) {
            rotation_target.rotation = camera_transform.rotation;
        }

        camera_transform.rotation = rotation_target.rotation;
        camera_transform.rotation = camera_transform.rotation.normalize();
    }
}