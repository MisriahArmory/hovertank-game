use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    bundles::input::InputBundle,
    components::{CameraFocus, Hover, LocalPlayer},
};

#[derive(Bundle)]
pub struct LocalPlayerBundle {
    pub hover: Hover,
    pub external_force: ExternalForce,
    pub ray_caster: RayCaster,
    pub player_model: PbrBundle,
    pub input: InputBundle,
    pub local_player: LocalPlayer,
    pub camera_focus: CameraFocus,
}
