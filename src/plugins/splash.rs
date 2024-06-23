use bevy::prelude::*;

use crate::{
    components::splash::OnSplash,
    states::app::AppState,
    systems::{
        despawn_with_component,
        splash::{setup_splash, splash},
    },
};

pub fn splash_plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Splash), setup_splash)
        .add_systems(Update, splash.run_if(in_state(AppState::Splash)))
        .add_systems(OnExit(AppState::Splash), despawn_with_component::<OnSplash>);
}
