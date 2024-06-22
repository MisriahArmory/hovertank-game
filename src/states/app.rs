use bevy::prelude::*;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Splash,
    MainMenu,
    Loading,
    InGame,
}
