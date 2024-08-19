use bevy::prelude::*;

use crate::{components::LocalPlayer, traits::StableInterpolate};

/// Rotate the player model to align with the third person camera
pub fn rotate_local_player(
    mut set: ParamSet<(
        Query<&mut Transform, With<LocalPlayer>>,
        Query<&Transform, With<Camera>>,
    )>,
    time: Res<Time>,
) {
    let camera_transform_query = set.p1();
    let camera_transform = camera_transform_query.single();
    let camera_rotation = camera_transform.rotation;
    let mut player_transform_query = set.p0();
    let mut player_transform = player_transform_query.single_mut();

    player_transform
        .rotation
        .smooth_nudge(&camera_rotation, 16.0, time.delta_seconds());
}
