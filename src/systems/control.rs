use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    components::{LocalPlayer, ThirdPersonCamera, ThirdPersonCameraFocus},
    constants::movement::MOVEMENT_SPEED,
    key_mappings::movement_key_mapping::MoveAction,
};

/// System to handle input controls.  Looks for an entity with an assigned input mapping
/// and will scan against the controls to determine if the entity should be moved.
pub fn control(
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

    // Get the camera view axis
    let forward = camera_transform.forward();
    let right = camera_transform.right();

    // Project onto the x-z plane
    let forward_xz = Vec3::new(forward.x, 0.0, forward.z).normalize();
    let right_xz = Vec3::new(right.x, 0.0, right.z).normalize();

    let mut query = set.p1();

    for (mut transform, action) in query.iter_mut() {
        let mut movement = Vec3::ZERO;

        for pressed_action in action.get_pressed() {
            match pressed_action {
                MoveAction::Left => movement -= right_xz,
                MoveAction::Right => movement += right_xz,
                MoveAction::Forward => movement += forward_xz,
                MoveAction::Backward => movement -= forward_xz,
            }
        }

        if movement != Vec3::ZERO {
            movement = movement.normalize();
            transform.translation += movement * MOVEMENT_SPEED * time.delta_seconds();
        }
    }
}
