use avian3d::prelude::*;
use bevy::prelude::*;

use crate::components::{LocalPlayer, RotationProperties};

/// Rotate the player model to align with the third person camera
pub fn rotate_local_player(
    mut set: ParamSet<(
        Query<
            (
                &mut ExternalTorque,
                &Inertia,
                &Transform,
                &RotationProperties,
            ),
            With<LocalPlayer>,
        >,
        Query<&Transform, With<Camera>>,
    )>,
) {
    let camera_rotation = set.p1().single().rotation;
    let mut player_query = set.p0();
    let Ok((mut player_torque, player_inertia, player_transform, player_rotation_properties)) =
        player_query.get_single_mut()
    else {
        return;
    };

    let rotation_diff = camera_rotation * player_transform.rotation.inverse();
    let (axis, angle) = rotation_diff.to_axis_angle();
    let torque =
        player_inertia.mul_vec3(axis * angle) * player_rotation_properties.rotation_strength;

    player_torque.apply_torque(torque);
}
