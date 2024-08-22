use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{key_mappings::camera::ChangeCameraMode, states::camera_mode::CameraMode};

pub fn toggle_camera_mode(
    query: Query<&ActionState<ChangeCameraMode>>,
    current_state: Res<State<CameraMode>>,
    mut next_state: ResMut<NextState<CameraMode>>,
) {
    for action in query.iter() {
        for pressed_action in action.get_just_pressed() {
            match pressed_action {
                ChangeCameraMode::ToggleMode => next_state.set(match current_state.get() {
                    CameraMode::ThirdPerson => CameraMode::FirstPerson,
                    CameraMode::FirstPerson => CameraMode::ThirdPerson,
                }),
                ChangeCameraMode::FirstPerson => next_state.set(CameraMode::FirstPerson),
                ChangeCameraMode::ThirdPerson => next_state.set(CameraMode::ThirdPerson),
            }
        }
    }
}
