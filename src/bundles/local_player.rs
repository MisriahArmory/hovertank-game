use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    components::{InGame, LocalPlayer, ThirdPersonCameraFocus},
    key_mappings::movement_key_mapping::MoveAction,
};

#[derive(Bundle)]
pub struct LocalPlayerBundle {
    pub player_model: PbrBundle,
    pub in_game: InGame,
    pub input_manager: InputManagerBundle<MoveAction>,
    pub local_player: LocalPlayer,
    pub third_person_camera_focus: ThirdPersonCameraFocus,
}
