use bevy::prelude::*;

#[derive(Component, Debug, Default)]
pub struct DampenRotation {
    /// How aggressively should the entity attempt to dampen rotation in each axis
    pub strength: Vec3,
}
