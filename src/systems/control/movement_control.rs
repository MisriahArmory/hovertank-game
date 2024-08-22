use avian3d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    components::{CameraFocus, LocalPlayer},
    key_mappings::movement::MoveAction,
    traits::Project,
};

/// System to handle input controls.  Looks for an entity with an assigned input mapping
/// and will scan against the controls to determine if the entity should be moved.
pub fn movement_control(
    mut set: ParamSet<(
        Query<&Transform, With<Camera>>,
        Query<
            (&mut ExternalForce, &Mass, &ActionState<MoveAction>),
            (With<LocalPlayer>, With<CameraFocus>),
        >,
    )>,
) {
    let camera_query = set.p0();
    let camera_transform = camera_query.single();

    // Get the camera view axes on the xz plane
    let forward = camera_transform.forward().project_normalized(Vec3::Y);
    let right = camera_transform.right().project_normalized(Vec3::Y);

    let mut query = set.p1();

    for (mut force, mass, action) in query.iter_mut() {
        let mut movement = Vec3::ZERO;

        for pressed_action in action.get_pressed() {
            match pressed_action {
                MoveAction::Left => movement -= right * mass.0 * 4.0,
                MoveAction::Right => movement += right * mass.0 * 4.0,
                MoveAction::Forward => movement += forward * mass.0 * 4.0,
                MoveAction::Backward => movement -= forward * mass.0 * 4.0,
                _ => {}
            }
        }

        let ax_data = action.clamped_axis_pair(&MoveAction::Move);
        movement += right * ax_data.x * mass.0 * 4.0;
        movement += forward * ax_data.y * mass.0 * 4.0;

        if movement != Vec3::ZERO {
            force.apply_force(movement);
        }
    }
}
