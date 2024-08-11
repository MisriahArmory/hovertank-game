use bevy::prelude::*;

use crate::states::{app::AppState, camera_mode::CameraMode, in_game::InGame};

pub fn state_plugin(app: &mut App) {
    app.enable_state_scoped_entities::<AppState>()
        .add_sub_state::<InGame>()
        .add_sub_state::<CameraMode>()
        .enable_state_scoped_entities::<InGame>()
        .enable_state_scoped_entities::<CameraMode>();
}
