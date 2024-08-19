use bevy::prelude::*;

use crate::{
    states::{app::AppState, camera_mode::CameraMode, in_game::InGame},
    systems::{
        camera_mode::toggle_camera_mode,
        cursor::{grab_cursor, release_cursor},
        first_person_camera::first_person_camera,
        in_game::{in_game, setup_in_game},
        in_game_menu::{setup_in_game_menu, toggle_in_game_menu},
        rotate_local_player::rotate_local_player,
        sets::ControlSet,
        third_person_camera::third_person_camera,
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
                third_person_camera
                    .after(ControlSet)
                    .run_if(in_state(CameraMode::ThirdPerson)),
                rotate_local_player
                    .after(ControlSet)
                    .run_if(in_state(CameraMode::ThirdPerson)),
                first_person_camera
                    .after(ControlSet)
                    .run_if(in_state(CameraMode::FirstPerson)),
                (in_game, toggle_in_game_menu).run_if(in_state(AppState::InGame)),
                grab_cursor.run_if(in_state(InGame::Running)),
                toggle_camera_mode.run_if(in_state(InGame::Running)),
            ),
        )
        .add_systems(OnExit(AppState::InGame), release_cursor);
}
