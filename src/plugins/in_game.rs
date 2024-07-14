use bevy::prelude::*;

use crate::{
    plugins::control::control_plugin,
    states::{app::AppState, in_game::InGame as InGameState},
    systems::{
        camera::camera,
        cursor::{grab_cursor, release_cursor},
        in_game::{in_game, setup_in_game},
        in_game_menu::{setup_in_game_menu, toggle_in_game_menu},
    },
};

pub fn in_game_plugin(app: &mut App) {
    app.add_sub_state::<InGameState>()
        .enable_state_scoped_entities::<InGameState>()
        .enable_state_scoped_entities::<AppState>()
        .add_systems(OnEnter(AppState::InGame), setup_in_game)
        .add_systems(
            OnEnter(InGameState::MenuOpen),
            (setup_in_game_menu, release_cursor),
        )
        .add_systems(
            Update,
            (in_game, toggle_in_game_menu).run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            Update,
            (grab_cursor, camera).run_if(in_state(InGameState::Running)),
        )
        .add_plugins(control_plugin);
}
