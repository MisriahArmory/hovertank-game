use bevy::prelude::*;

use crate::{
    bundles::input::InputBundle,
    components::{InGame, LocalPlayer, ThirdPersonCameraFocus},
};

#[derive(Bundle)]
pub struct LocalPlayerBundle {
    pub player_model: PbrBundle,
    pub in_game: InGame,
    pub input: InputBundle,
    pub local_player: LocalPlayer,
    pub third_person_camera_focus: ThirdPersonCameraFocus,
}
