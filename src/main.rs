use bevy::prelude::*;
use hovertank_game::{
    events::log_transition::log_transitions,
    plugins::{loading::loading_plugin, main_menu::main_menu_plugin, splash::splash_plugin},
    states::app::AppState,
    systems::setup,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        .add_systems(Update, log_transitions)
        .add_plugins((splash_plugin, main_menu_plugin, loading_plugin))
        .run();
}
