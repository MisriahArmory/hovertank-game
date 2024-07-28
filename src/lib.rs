#![allow(clippy::type_complexity)]

use bevy::math::Quat;
pub mod bundles;
pub mod components;
pub mod constants;
pub mod events;
pub mod key_mappings;
pub mod plugins;
pub mod resources;
pub mod states;
pub mod systems;
pub mod traits;

pub type Rotor3 = Quat;
