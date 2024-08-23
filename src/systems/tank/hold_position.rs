use avian3d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{components::HoldPosition, key_mappings::movement::MoveAction};

pub fn hold_position(
    mut query: Query<(
        &mut ExternalForce,
        &LinearVelocity,
        &Mass,
        &HoldPosition,
        Option<&ActionState<MoveAction>>,
    )>,
) {
    for (mut force, velocity, mass, hold_position, action) in query.iter_mut() {
        if let Some(action_state) = action {
            let ax_data = action_state.clamped_axis_pair(&MoveAction::Move);
            if !action_state.get_pressed().is_empty() || ax_data != Vec2::ZERO {
                continue;
            }
        }

        if velocity.0.length() == 0.0 {
            continue;
        }

        // Multiplication is element-wise in glam
        let stabilize_force = (-mass.0 * velocity.0) * hold_position.strength;
        force.apply_force(stabilize_force);
    }
}
