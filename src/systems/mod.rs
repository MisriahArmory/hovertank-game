pub mod camera_follow;
pub mod cursor;
pub mod in_game;
pub mod in_game_menu;
pub mod loading;
pub mod main_menu;
pub mod movement_control;
pub mod rotation_control;
pub mod sets;
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
