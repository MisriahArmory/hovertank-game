use bevy::prelude::*;
use hovertank_game::{
    commands::{setup, setup_menu},
    events::log_transition::log_transitions,
    plugins::splash::splash_plugin,
    states::app::AppState,
    systems::menu::menu,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        .add_plugins(splash_plugin)
        .add_systems(Update, log_transitions)
        .add_systems(OnEnter(AppState::MainMenu), setup_menu)
        .add_systems(Update, menu.run_if(in_state(AppState::MainMenu)))
        .run();
}
