use bevy::prelude::*;
use leafwing_input_manager::plugin::InputManagerPlugin;

use crate::{
    components::LocalPlayer,
    key_mappings::{
        camera::ChangeCameraMode, in_game_ui::InGameUiAction, movement::MoveAction,
        rotation::RotationAction,
    },
    states::{camera_mode::CameraMode, in_game::InGame},
    systems::{
        movement_control::movement_control, rotation_control::rotation_control, sets::ControlSet,
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
                rotation_control::<Camera>.run_if(in_state(CameraMode::ThirdPerson)),
                rotation_control::<LocalPlayer>.run_if(in_state(CameraMode::FirstPerson)),
            )
                .in_set(ControlSet),
        )
        .configure_sets(Update, ControlSet.run_if(in_state(InGame::Running)));
}
