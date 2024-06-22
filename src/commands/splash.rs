use bevy::prelude::*;

use crate::{components::splash::OnSplash, resources::timer::SplashTimer};

pub fn setup_splash(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            OnSplash,
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
