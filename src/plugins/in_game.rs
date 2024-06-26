use bevy::prelude::*;

use crate::{
    components::in_game::InGame,
    states::app::AppState,
    systems::{
        despawn_with_component,
        in_game::{in_game, setup_in_game},
    },
};

pub fn in_game_plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::InGame), setup_in_game)
        .add_systems(Update, in_game.run_if(in_state(AppState::InGame)))
        .add_systems(OnExit(AppState::InGame), despawn_with_component::<InGame>);
}
