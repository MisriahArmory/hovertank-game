use crate::constants::camera::{
    CAMERA_MAX_PITCH, CAMERA_MAX_PITCH_SPEED, CAMERA_MAX_YAW_SPEED, CAMERA_MIN_PITCH,
    CAMERA_PITCH_SPEED, CAMERA_YAW_SPEED,
};
use crate::{
    components::{LocalPlayer, ThirdPersonCamera},
    key_mappings::rotation::RotationAction,
    traits::Project,
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

    if let Some(ax_data) = action.axis_pair(&RotationAction::Rotate) {
        let yaw_rotation = match (ax_data.x().abs(), ax_data.x().signum()) {
            (0.0, 1.0) => Quat::IDENTITY,
            (_, -1.0) => Quat::from_axis_angle(Vec3::Y, 1.0),
            (_, 1.0) => Quat::from_axis_angle(Vec3::Y, -1.0),
            _ => Quat::IDENTITY,
        };

        let yaw_speed = (ax_data.x().abs() * CAMERA_YAW_SPEED).min(CAMERA_MAX_YAW_SPEED);
        camera_transform.rotation = camera_transform.rotation.slerp(
            yaw_rotation * camera_transform.rotation,
            yaw_speed * time.delta_seconds(),
        );
        camera_transform.rotation = camera_transform.rotation.normalize();

        // project the "right" vector onto the XZ plane to get our pitch axis
        let pitch_axis = camera_transform.right().project_normalized(Vec3::Y);
        let pitch_rotation = match (ax_data.y().abs(), ax_data.y().signum()) {
            (0.0, 1.0) => Quat::IDENTITY,
            (_, -1.0) => Quat::from_axis_angle(pitch_axis, 1.0),
            (_, 1.0) => Quat::from_axis_angle(pitch_axis, -1.0),
            _ => Quat::IDENTITY,
        };

        let pitch_target = pitch_rotation * camera_transform.rotation;
        let pitch_speed = (ax_data.y().abs() * CAMERA_PITCH_SPEED).min(CAMERA_MAX_PITCH_SPEED);
        let new_rotation = camera_transform
            .rotation
            .slerp(pitch_target, pitch_speed * time.delta_seconds());
        let mut rotation_target = camera_transform.with_rotation(new_rotation);

        let forward = rotation_target.forward();
        let forward_xz = Vec3::new(forward.x, 0.0, forward.z);
        let pitch = forward.angle_between(forward_xz);

        if !(CAMERA_MIN_PITCH..CAMERA_MAX_PITCH).contains(&pitch) {
            rotation_target.rotation = camera_transform.rotation;
        }

        camera_transform.rotation = rotation_target.rotation;
        camera_transform.rotation = camera_transform.rotation.normalize();
    }
}
