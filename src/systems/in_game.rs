use bevy::prelude::*;

use crate::{
    bundles::{input::InputBundle, local_player::LocalPlayerBundle},
    components::{LocalPlayer, ThirdPersonCamera, ThirdPersonCameraFocus},
    constants::camera::{CAMERA_FOLLOW_DISTANCE, CAMERA_FOLLOW_HEIGHT},
    states::app::AppState,
};

pub fn setup_in_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(100.0, 100.0)),
            material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
            ..default()
        },
    ));

    // Player
    commands.spawn((
        StateScoped(AppState::InGame),
        LocalPlayerBundle {
            local_player: LocalPlayer,
            player_model: PbrBundle {
                mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
                material: materials.add(Color::srgb(0.8, 0.7, 0.6)),
                transform: Transform::from_xyz(0.0, 0.5, 0.0),
                ..default()
            },
            input: InputBundle::default(),
            third_person_camera_focus: ThirdPersonCameraFocus::default(),
        },
    ));

    // Camera
    commands.spawn((
        StateScoped(AppState::InGame),
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, CAMERA_FOLLOW_HEIGHT, CAMERA_FOLLOW_DISTANCE),
            ..default()
        },
        ThirdPersonCamera,
    ));
}

pub fn in_game() {}
