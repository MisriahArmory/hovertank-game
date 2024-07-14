use bevy::prelude::*;
use leafwing_input_manager::plugin::InputManagerPlugin;

use crate::{
    key_mappings::{in_game_ui::InGameUiAction, movement::MoveAction},
    states::in_game::InGame,
    systems::movement_control::movement_control,
};

pub fn control_plugin(app: &mut App) {
    app.add_plugins(InputManagerPlugin::<MoveAction>::default())
        .add_plugins(InputManagerPlugin::<InGameUiAction>::default())
        .add_systems(Update, movement_control.run_if(in_state(InGame::Running)));
}
