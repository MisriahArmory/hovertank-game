use avian3d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{components::DampenRotation, key_mappings::rotation::RotationAction, traits::Dampen};

pub fn dampen_rotation(
    mut query: Query<(
        &mut ExternalTorque,
        &AngularVelocity,
        &Inertia,
        &DampenRotation,
        Option<&ActionState<RotationAction>>,
    )>,
) {
    for (mut torque, angular_velocity, inertia, dampen_rotation, action) in query.iter_mut() {
        if let Some(a) = action {
            let ax_data = a.clamped_axis_pair(&RotationAction::Rotate);
            if !a.get_pressed().is_empty() || ax_data != Vec2::ZERO {
                continue;
            }
        }

        if angular_velocity.0.length() == 0.0 {
            continue;
        }

        let dampen_torque = angular_velocity.dampen(inertia, dampen_rotation.strength);
        torque.apply_torque(dampen_torque);
    }
}
