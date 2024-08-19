use bevy::prelude::*;

use crate::{
    bundles::input::InputBundle,
    components::{CameraFocus, LocalPlayer},
};

#[derive(Bundle)]
pub struct LocalPlayerBundle {
    pub player_model: PbrBundle,
    pub input: InputBundle,
    pub local_player: LocalPlayer,
    pub camera_focus: CameraFocus,
}
