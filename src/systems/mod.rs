pub mod camera;
pub mod control;
pub mod in_game;
pub mod loading;
pub mod main_menu;
pub mod splash;

use bevy::prelude::*;

pub fn despawn_with_component<T: Component>(
    to_despawn: Query<Entity, With<T>>,
    mut commands: Commands,
) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
