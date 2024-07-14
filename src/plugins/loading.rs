use bevy::prelude::*;

use crate::{
    states::app::AppState,
    systems::loading::{loading, setup_loading},
};

pub fn loading_plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Loading), setup_loading)
        .add_systems(Update, loading.run_if(in_state(AppState::Loading)));
}
