use bevy::prelude::*;

pub mod menu;
pub mod splash;

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
