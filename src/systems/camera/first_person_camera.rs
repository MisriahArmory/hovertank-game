use bevy::prelude::*;

use crate::components::CameraFocus;

pub fn first_person_camera(
    mut set: ParamSet<(
        Query<&mut Transform, With<Camera>>,
        Query<&Transform, With<CameraFocus>>,
    )>,
) {
    let focus_transform_query = set.p1();
    let focus_transform = focus_transform_query.single();
    let focus_up: Vec3 = focus_transform.up().into();
    let camera_target_translation = focus_transform.translation + focus_up * 0.75;
    let camera_target_rotation = focus_transform.rotation;

    let mut camera_transform_query = set.p0();
    let mut camera_transform = camera_transform_query.single_mut();

    camera_transform.translation = camera_target_translation;
    camera_transform.rotation = camera_target_rotation;
}
