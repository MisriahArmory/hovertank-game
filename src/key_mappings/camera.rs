use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum ChangeCameraMode {
    FirstPerson,
    ThirdPerson,
    ToggleMode,
}

impl ChangeCameraMode {
    pub fn default_mapping() -> InputMap<Self> {
        InputMap::default()
            .with(Self::ToggleMode, KeyCode::KeyR)
            .with(Self::ToggleMode, GamepadButtonType::LeftThumb)
    }
}
