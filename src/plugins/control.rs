use bevy::prelude::*;
use leafwing_input_manager::plugin::InputManagerPlugin;

use crate::{
    key_mappings::movement_key_mapping::MoveAction, states::app::AppState,
    systems::control::control,
};

pub fn control_plugin(app: &mut App) {
    app.add_plugins(InputManagerPlugin::<MoveAction>::default())
        .add_systems(Update, control.run_if(in_state(AppState::InGame)));
}
