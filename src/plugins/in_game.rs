use bevy::prelude::*;

use crate::{
    states::{app::AppState, in_game::InGame},
    systems::{
        camera_follow_focus::camera_follow_focus,
        camera_orbit_focus::camera_orbit_focus,
        cursor::{grab_cursor, release_cursor},
        in_game::{in_game, setup_in_game},
        in_game_menu::{setup_in_game_menu, toggle_in_game_menu},
        movement_control::movement_control,
        rotation_control::rotation_control,
    },
};

pub fn in_game_plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::InGame), setup_in_game)
        .add_systems(
            OnEnter(InGame::MenuOpen),
            (setup_in_game_menu, release_cursor),
        )
        .add_systems(
            Update,
            (
                camera_orbit_focus.after(rotation_control),
                camera_follow_focus.after(movement_control),
            )
                .chain()
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            Update,
            (in_game, toggle_in_game_menu).run_if(in_state(AppState::InGame)),
        )
        .add_systems(Update, grab_cursor.run_if(in_state(InGame::Running)))
        .add_systems(OnExit(AppState::InGame), release_cursor);
}
