use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    components::{LocalPlayer, ThirdPersonCamera, ThirdPersonCameraFocus},
    constants::movement::MOVEMENT_SPEED,
    key_mappings::movement::MoveAction,
    traits::Project,
};

/// System to handle input controls.  Looks for an entity with an assigned input mapping
/// and will scan against the controls to determine if the entity should be moved.
pub fn movement_control(
    mut set: ParamSet<(
        Query<&Transform, With<ThirdPersonCamera>>,
        Query<
            (&mut Transform, &ActionState<MoveAction>),
            (With<LocalPlayer>, With<ThirdPersonCameraFocus>),
        >,
    )>,
    time: Res<Time>,
) {
    let camera_query = set.p0();
    let camera_transform = camera_query.single();

    // Get the camera view axes on the xz plane
    let forward = camera_transform.forward().project_normalized(Vec3::Y);
    let right = camera_transform.right().project_normalized(Vec3::Y);

    let mut query = set.p1();

    for (mut transform, action) in query.iter_mut() {
        let mut movement = Vec3::ZERO;

        for pressed_action in action.get_pressed() {
            match pressed_action {
                MoveAction::Left => movement -= right,
                MoveAction::Right => movement += right,
                MoveAction::Forward => movement += forward,
                MoveAction::Backward => movement -= forward,
            }
        }

        if movement != Vec3::ZERO {
            movement = movement.normalize();
            transform.translation += movement * MOVEMENT_SPEED * time.delta_seconds();
        }
    }
}
