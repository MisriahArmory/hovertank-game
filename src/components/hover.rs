use bevy::prelude::*;

/// Something that should be attempting to hover.  Hovering force is only applied when the
/// entity's height is below its target.
#[derive(Component, Default)]
pub struct Hover {
    /// The desired distance from the ground the entity should try to hover
    pub target_height: f32,
    /// Scalar value to multiply the necessary force to move the entity toward its target height
    pub max_hover_strength: f32,
    /// Maximum speed to return to the target height
    pub max_hover_speed: f32,
}
