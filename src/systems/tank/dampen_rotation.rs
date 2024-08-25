use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{components::DampenRotation, traits::Dampen};

pub fn dampen_rotation(
    mut query: Query<(
        &mut ExternalTorque,
        &AngularVelocity,
        &Inertia,
        &DampenRotation,
    )>,
) {
    for (mut torque, angular_velocity, inertia, dampen_rotation) in query.iter_mut() {
        if angular_velocity.0.length() == 0.0 {
            continue;
        }

        let dampen_torque = angular_velocity.dampen(inertia, dampen_rotation.strength);
        torque.apply_torque(dampen_torque);
    }
}
