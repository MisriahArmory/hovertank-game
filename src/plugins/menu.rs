use bevy::prelude::*;

use crate::{
    commands::menu::setup_menu,
    components::menu::OnMainMenu,
    states::app::AppState,
    systems::{despawn_with_component, menu::menu},
};

pub fn menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::MainMenu), setup_menu)
        .add_systems(Update, menu.run_if(in_state(AppState::MainMenu)))
        .add_systems(
            OnExit(AppState::MainMenu),
            despawn_with_component::<OnMainMenu>,
        );
}
