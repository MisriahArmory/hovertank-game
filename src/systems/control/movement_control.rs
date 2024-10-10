use avian3d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    components::{CameraFocus, LocalPlayer, MovementProperties},
    key_mappings::movement::MoveAction,
    traits::Project,
};

/// System to handle input controls.  Looks for an entity with an assigned input mapping
/// and will scan against the controls to determine if the entity should be moved.
pub fn movement_control(
    camera_query: Query<&Transform, With<Camera>>,
    mut query: Query<
        (
            &mut ExternalForce,
            &Mass,
            &ActionState<MoveAction>,
            &MovementProperties,
        ),
        (With<LocalPlayer>, With<CameraFocus>),
    >,
) {
    let camera_transform = camera_query.single();

    // Get the camera view axes on the xz plane
    let forward = camera_transform.forward().project_normalized(Vec3::Y);
    let right = camera_transform.right().project_normalized(Vec3::Y);

    for (mut force, mass, action, movement_properties) in query.iter_mut() {
        let mut movement = Vec3::ZERO;

        for pressed_action in action.get_pressed() {
            match pressed_action {
                MoveAction::Left => {
                    movement -= right * mass.0 * movement_properties.left_move_strength
                }
                MoveAction::Right => {
                    movement += right * mass.0 * movement_properties.right_move_strength
                }
                MoveAction::Forward => {
                    movement += forward * mass.0 * movement_properties.forward_move_strength
                }
                MoveAction::Backward => {
                    movement -= forward * mass.0 * movement_properties.backward_move_strength
                }
                _ => {}
            }
        }

        let ax_data = action.clamped_axis_pair(&MoveAction::Move);

        let x_move_strength = match ax_data.x {
            ..0.0 => movement_properties.left_move_strength,
            _ => movement_properties.right_move_strength,
        };

        let y_move_strength = match ax_data.y {
            ..0.0 => movement_properties.backward_move_strength,
            _ => movement_properties.forward_move_strength,
        };

        movement += right * ax_data.x * mass.0 * x_move_strength;
        movement += forward * ax_data.y * mass.0 * y_move_strength;

        force.apply_force(movement);
    }
}
