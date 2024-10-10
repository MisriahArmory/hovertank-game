use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    components::{CameraFocus, LocalPlayer},
    constants::camera::{
        CAMERA_FOLLOW_DISTANCE, CAMERA_FOLLOW_HEIGHT, CAMERA_FOLLOW_PITCH_ROTATION_SPEED,
        CAMERA_FOLLOW_SNAP_DISTANCE, CAMERA_FOLLOW_SPEED, CAMERA_FOLLOW_YAW_ROTATION_SPEED,
    },
    key_mappings::rotation::RotationAction,
    traits::{Orbit, StableInterpolate},
    Rotor3,
};

pub fn third_person_camera(
    control_query: Query<&ActionState<RotationAction>, With<LocalPlayer>>,
    mut set: ParamSet<(
        Query<&mut Transform, With<Camera>>,
        Query<(&Transform, &mut CameraFocus)>,
    )>,
    time: Res<Time>,
) {
    let mut focus_query = set.p1();
    let (focus_transform, mut target_look_at) = focus_query.single_mut();

    let focus_offset = focus_transform.translation + Vec3::Y * CAMERA_FOLLOW_HEIGHT;
    target_look_at
        .target_look_at_point
        .smooth_nudge(&focus_offset, 32.0, time.delta_seconds());

    let target_offset = target_look_at.target_look_at_point;

    let mut camera_transform_query = set.p0();
    let mut camera_transform = camera_transform_query.single_mut();

    // Neutralize any camera roll
    let right = camera_transform.right();
    let right_xz = Vec3::new(right.x, 0.0, right.z);
    let neutralize_roll_rotor = Rotor3::from_rotation_arc(right.into(), right_xz);
    camera_transform.rotation *= neutralize_roll_rotor;

    let relative_offset_translation = target_offset - camera_transform.translation;
    let relative_offset_translation_direction = relative_offset_translation.normalize();

    let camera_target_point =
        target_offset - relative_offset_translation_direction * CAMERA_FOLLOW_DISTANCE;

    let follow_speed =
        CAMERA_FOLLOW_SPEED * relative_offset_translation.length() / CAMERA_FOLLOW_DISTANCE;

    let camera_target_distance = (camera_transform.translation - camera_target_point).length();

    // Smoothly move the camera toward the target point
    if camera_target_distance > CAMERA_FOLLOW_SNAP_DISTANCE {
        camera_transform.translation.smooth_nudge(
            &camera_target_point,
            follow_speed,
            time.delta_seconds(),
        );
    } else {
        // Snap to avoid jittering
        camera_transform.translation = camera_target_point;
    }

    let action = control_query.single();
    let ax_data = action.clamped_axis_pair(&RotationAction::Rotate);

    // Orbit around the focus offset point
    let camera_follow_yaw_rotation_speed =
        32.0 + CAMERA_FOLLOW_YAW_ROTATION_SPEED * ax_data.x.abs().min(1.0);
    let camera_forward = camera_transform.forward();

    camera_transform.translation = camera_transform.translation.orbit(
        target_offset,
        camera_forward.into(),
        Vec3::Y,
        camera_follow_yaw_rotation_speed,
        time.delta_seconds(),
    );

    camera_transform.translation = camera_transform.translation.orbit(
        target_offset,
        camera_forward.into(),
        camera_transform.right().into(),
        CAMERA_FOLLOW_PITCH_ROTATION_SPEED,
        time.delta_seconds(),
    );
}
