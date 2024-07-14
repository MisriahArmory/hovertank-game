use bevy::prelude::*;

use crate::{
    states::app::AppState,
    systems::main_menu::{main_menu, setup_main_menu},
};

pub fn main_menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
        .add_systems(Update, main_menu.run_if(in_state(AppState::MainMenu)));
}
