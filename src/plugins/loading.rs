use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_asset_loader::loading_state::{config::ConfigureLoadingState, LoadingState, LoadingStateAppExt, LoadingStateSet};
use iyes_progress::{ProgressPlugin, ProgressSystem};

use crate::{
    components::loading::OnLoading, resources::assets::ImageAssets, states::app::AppState, systems::{
        despawn_with_component,
        loading::{loading, print_progress, setup_loading},
    }
};

pub fn loading_plugin(app: &mut App) {
    app
        .add_plugins((
            ProgressPlugin::new(AppState::Loading).continue_to(AppState::InGame),
            FrameTimeDiagnosticsPlugin,
        ))
        .add_loading_state(
            LoadingState::new(AppState::Loading)
                .load_collection::<ImageAssets>(),
        )
        .add_systems(OnEnter(AppState::Loading), setup_loading)
        .add_systems(
            Update,
            (loading.track_progress(), print_progress)
                .chain()
                .run_if(in_state(AppState::Loading))
                .after(LoadingStateSet(AppState::Loading))
        )
        .add_systems(
            OnExit(AppState::Loading),
            despawn_with_component::<OnLoading>,
        );
}
