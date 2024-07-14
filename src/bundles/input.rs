use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::key_mappings::{in_game_ui::InGameUiAction, movement::MoveAction};

#[derive(Bundle)]
pub struct InputBundle {
    pub movement_input_manager: InputManagerBundle<MoveAction>,
    pub ingame_ui_input_manager: InputManagerBundle<InGameUiAction>,
}

impl Default for InputBundle {
    fn default() -> Self {
        Self {
            movement_input_manager: InputManagerBundle::with_map(MoveAction::default_mapping()),
            ingame_ui_input_manager: InputManagerBundle::with_map(InGameUiAction::default_mapping()),
        }
    }
}
