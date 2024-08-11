use bevy::prelude::*;
use leafwing_input_manager::plugin::InputManagerPlugin;

use crate::{
    key_mappings::{
        camera::ChangeCameraMode, in_game_ui::InGameUiAction, movement::MoveAction,
        rotation::RotationAction,
    },
    states::{camera_mode::CameraMode, in_game::InGame},
    systems::{
        movement_control::movement_control, sets::ControlSet,
        third_person_rotation_control::third_person_rotation_control,
    },
};

pub fn control_plugin(app: &mut App) {
    app.add_plugins(InputManagerPlugin::<MoveAction>::default())
        .add_plugins(InputManagerPlugin::<InGameUiAction>::default())
        .add_plugins(InputManagerPlugin::<RotationAction>::default())
        .add_plugins(InputManagerPlugin::<ChangeCameraMode>::default())
        .add_systems(
            Update,
            (
                movement_control,
                third_person_rotation_control.run_if(in_state(CameraMode::ThirdPerson)),
            )
                .in_set(ControlSet),
        )
        .configure_sets(Update, ControlSet.run_if(in_state(InGame::Running)));
}
