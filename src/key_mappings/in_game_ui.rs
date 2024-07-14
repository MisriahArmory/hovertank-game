use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum InGameUiAction {
    ToggleMenu,
}

impl InGameUiAction {
    pub fn default_mapping() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        input_map.insert(Self::ToggleMenu, KeyCode::Escape);

        input_map
    }
}
