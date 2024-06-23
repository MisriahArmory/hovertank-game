use bevy::prelude::*;
use hovertank_game::{
    events::log_transition::log_transitions,
    plugins::{
        in_game::in_game_plugin, loading::loading_plugin, main_menu::main_menu_plugin,
        splash::splash_plugin,
    },
    states::app::AppState,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_systems(Update, log_transitions)
        .add_plugins((
            splash_plugin,
            main_menu_plugin,
            loading_plugin,
            in_game_plugin,
        ))
        .run();
}
