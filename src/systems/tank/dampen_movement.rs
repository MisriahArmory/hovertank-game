use avian3d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{components::DampenMovement, key_mappings::movement::MoveAction, traits::Dampen};

pub fn dampen_movement(
    mut query: Query<(
        &mut ExternalForce,
        &LinearVelocity,
        &Mass,
        &DampenMovement,
        Option<&ActionState<MoveAction>>,
    )>,
) {
    for (mut force, velocity, mass, dampen_movement, action) in query.iter_mut() {
        if let Some(action_state) = action {
            let ax_data = action_state.clamped_axis_pair(&MoveAction::Move);
            if !action_state.get_pressed().is_empty() || ax_data != Vec2::ZERO {
                continue;
            }
        }

        if velocity.0.length() == 0.0 {
            continue;
        }

        let dampen_force = velocity.dampen(mass, dampen_movement.strength);
        force.apply_force(dampen_force);
    }
}
