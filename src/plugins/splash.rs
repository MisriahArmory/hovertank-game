use bevy::prelude::*;

use crate::{
    states::app::AppState,
    systems::splash::{setup_splash, splash},
};

pub fn splash_plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Splash), setup_splash)
        .add_systems(Update, splash.run_if(in_state(AppState::Splash)));
}
