use bevy::prelude::*;

#[derive(Component)]
pub struct HoldPosition {
    /// How aggressively should the entity attempt to hold position in each dimension
    pub strength: Vec3,
}
