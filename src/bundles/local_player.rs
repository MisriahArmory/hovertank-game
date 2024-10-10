use bevy::prelude::*;

use crate::{
    bundles::{input::InputBundle, tank::TankBundle},
    components::{CameraFocus, LocalPlayer},
};

#[derive(Bundle, Default)]
pub struct LocalPlayerBundle {
    pub tank: TankBundle,
    pub player_model: PbrBundle,
    pub input: InputBundle,
    pub local_player: LocalPlayer,
    pub camera_focus: CameraFocus,
}
