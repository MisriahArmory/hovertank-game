use bevy::prelude::*;

use crate::{
    components::menu::OnMainMenu,
    states::app::AppState,
    systems::{
        despawn_with_component,
        main_menu::{main_menu, setup_main_menu},
    },
};

pub fn main_menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
        .add_systems(Update, main_menu.run_if(in_state(AppState::MainMenu)))
        .add_systems(
            OnExit(AppState::MainMenu),
            despawn_with_component::<OnMainMenu>,
        );
}
