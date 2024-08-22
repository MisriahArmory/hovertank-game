use bevy::prelude::*;

/// Something that should be attempting to hover.  Hovering force is only applied when the
/// entity's height is below its target.
#[derive(Component)]
pub struct Hover {
    /// The desired distance from the ground the entity should try to hover
    pub target_height: f32,
    /// The height at which pre-emptive braking will be applied to soften the approach to the
    /// target height from above.
    pub braking_height: f32,
    /// Scalar value to multiply the necessary force to move the entity toward its target height
    pub max_hover_strength: f32,
    /// Scalar value to multiply the necessary force to approach its target height
    pub max_brake_strength: f32,
    /// Maximum speed to return to the target height
    pub max_hover_speed: f32,
}

impl Default for Hover {
    fn default() -> Self {
        Self {
            target_height: 1.5,
            braking_height: 3.0,
            max_hover_strength: 2.0,
            max_brake_strength: 0.5,
            max_hover_speed: 0.25,
        }
    }
}
