use crate::states::app::AppState;
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(AppState = AppState::InGame)]
pub enum InGame {
    #[default]
    Running,
    MenuOpen,
}
