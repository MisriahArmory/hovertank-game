use bevy::prelude::*;

use crate::{
    components::loading::OnLoading,
    states::app::AppState,
    systems::{
        despawn_with_component,
        loading::{loading, setup_loading},
    },
};

pub fn loading_plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Loading), setup_loading)
        .add_systems(Update, loading.run_if(in_state(AppState::Loading)))
        .add_systems(
            OnExit(AppState::Loading),
            despawn_with_component::<OnLoading>,
        );
}
