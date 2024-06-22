use bevy::prelude::*;

use crate::{resources::timer::SplashTimer, states::app::AppState};

pub fn splash(
    time: Res<Time>,
    mut app_state: ResMut<NextState<AppState>>,
    mut timer: ResMut<SplashTimer>,
) {
    // Placeholder until we have actual assets to load
    if timer.tick(time.delta()).finished() {
        app_state.set(AppState::MainMenu)
    }
}
