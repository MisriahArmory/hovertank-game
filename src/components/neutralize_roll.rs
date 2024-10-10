use bevy::prelude::*;

#[derive(Component, Debug, Default)]
pub struct NeutralizeRoll {
    /// How aggressively should the entity attempt to neutralize any existing roll in each of the
    /// torque axes
    pub strength: Vec3,
}
