use bevy::prelude::*;

use crate::{resources::timer::SplashTimer, states::app::AppState};

pub fn setup_splash(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((StateScoped(AppState::Splash), Camera2dBundle::default()));
    let icon = asset_server.load("branding/icon.png");
    commands
        .spawn((
            StateScoped(AppState::Splash),
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

    commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)));
}

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
