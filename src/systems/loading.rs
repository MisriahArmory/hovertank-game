use bevy::prelude::*;

use crate::{
    components::loading::OnLoading, resources::timer::LoadingTimer, states::app::AppState,
};
pub fn setup_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), OnLoading));
    let icon = asset_server.load("branding/icon.png");
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            OnLoading,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Px(200.0),
                    ..default()
                },
                image: UiImage::new(icon),
                ..default()
            });
        });

    commands.insert_resource(LoadingTimer(Timer::from_seconds(1.0, TimerMode::Once)));
}

pub fn loading(
    time: Res<Time>,
    mut app_state: ResMut<NextState<AppState>>,
    mut timer: ResMut<LoadingTimer>,
) {
    // Placeholder until we have actual assets to load
    if timer.tick(time.delta()).finished() {
        app_state.set(AppState::InGame)
    }
}
