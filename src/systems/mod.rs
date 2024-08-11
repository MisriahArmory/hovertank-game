pub mod camera_mode;
pub mod cursor;
pub mod first_person_camera;
pub mod in_game;
pub mod in_game_menu;
pub mod loading;
pub mod main_menu;
pub mod movement_control;
pub mod sets;
pub mod splash;
pub mod third_person_camera;
pub mod third_person_rotation_control;

use bevy::prelude::*;

pub fn despawn_with_component<T: Component>(
    to_despawn: Query<Entity, With<T>>,
    mut commands: Commands,
) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
