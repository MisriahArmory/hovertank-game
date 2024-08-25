use bevy::prelude::*;

#[derive(Component)]
pub struct DampenMovement {
    /// How aggressively should the entity attempt to dampen movement in each dimension
    pub strength: Vec3,
}
