use bevy::prelude::*;

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct CameraFocus {
    pub target_look_at_point: Vec3,
}
