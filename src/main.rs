use bevy::prelude::*;
use hovertank_game::{
    commands::setup,
    events::log_transition::log_transitions,
    plugins::{menu::menu_plugin, splash::splash_plugin},
    states::app::AppState,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        .add_systems(Update, log_transitions)
        .add_plugins((splash_plugin, menu_plugin))
        .run();
}
