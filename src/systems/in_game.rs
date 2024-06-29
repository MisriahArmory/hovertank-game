use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    bundles::local_player::LocalPlayerBundle,
    components::{InGame, LocalPlayer},
    key_mappings::movement_key_mapping::MoveAction,
};

pub fn setup_in_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 1.5, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        InGame,
    ));

    // Light
    commands.spawn((
        PointLightBundle {
            point_light: PointLight {
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        },
        InGame,
    ));

    // Plane
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(100.0, 100.0)),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
            ..default()
        },
        InGame,
    ));

    // Player
    commands.spawn(LocalPlayerBundle {
        local_player: LocalPlayer,
        in_game: InGame,
        player_model: PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        input_manager: InputManagerBundle::with_map(MoveAction::default_mapping()),
    });
}

pub fn in_game() {}
