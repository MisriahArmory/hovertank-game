use bevy::prelude::*;

#[derive(Deref, DerefMut, Resource)]
pub struct GameTimer(pub Timer);

#[derive(Deref, DerefMut, Resource)]
pub struct SplashTimer(pub Timer);
