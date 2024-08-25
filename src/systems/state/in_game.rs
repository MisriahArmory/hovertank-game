use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    bundles::{local_player::LocalPlayerBundle, tank::TankBundle},
    constants::camera::{CAMERA_FOLLOW_DISTANCE, CAMERA_FOLLOW_HEIGHT},
    states::app::AppState,
};

pub fn setup_in_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn((
        StateScoped(AppState::InGame),
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, CAMERA_FOLLOW_HEIGHT, CAMERA_FOLLOW_DISTANCE),
            ..default()
        },
    ));

    // Light
    commands.spawn((
        StateScoped(AppState::InGame),
        PointLightBundle {
            point_light: PointLight {
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        },
    ));

    // Plane
    commands.spawn((
        StateScoped(AppState::InGame),
        RigidBody::Static,
        Collider::trimesh_from_mesh(&Plane3d::default().mesh().size(100.0, 100.0).build()).unwrap(),
        CollisionMargin(0.1),
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(100.0, 100.0)),
            material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
            ..default()
        },
    ));

    // Dummy Cube
    commands.spawn((
        StateScoped(AppState::InGame),
        TankBundle::default(),
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(Color::srgb(0.8, 0.5, 0.4)),
            transform: Transform::from_xyz(0.0, 3.5, 0.1),
            ..default()
        },
    ));

    // Player
    commands.spawn((
        StateScoped(AppState::InGame),
        LocalPlayerBundle {
            player_model: PbrBundle {
                mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
                material: materials.add(Color::srgb(0.8, 0.7, 0.6)),
                transform: Transform::from_xyz(0.0, 1.5, 0.0),
                ..default()
            },
            ..default()
        },
    ));
}

pub fn in_game() {}
