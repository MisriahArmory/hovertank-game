use bevy::prelude::*;
use bevy_diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use hovertank_game::{
    events::log_transition::log_transitions,
    plugins::{
        in_game::in_game_plugin, loading::loading_plugin, main_menu::main_menu_plugin,
        splash::splash_plugin, state::state_plugin,
    },
    states::app::AppState,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((LogDiagnosticsPlugin::default(), FrameTimeDiagnosticsPlugin))
        .init_state::<AppState>()
        .add_systems(Update, log_transitions)
        .add_plugins((
            state_plugin,
            splash_plugin,
            main_menu_plugin,
            loading_plugin,
            in_game_plugin,
        ))
        .run();
}
