use crate::states::in_game::InGame;
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(InGame = InGame::Running)]
pub enum CameraMode {
    FirstPerson,
    #[default]
    ThirdPerson,
}
