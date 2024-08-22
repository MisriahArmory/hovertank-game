use avian3d::prelude::*;
use bevy::prelude::*;

use crate::components::{hold_position::HoldPosition, Hover};

#[derive(Bundle)]
pub struct TankBundle {
    hover: Hover,
    hold_position: HoldPosition,
    rigid_body: RigidBody,
    collider: Collider,
    collider_density: ColliderDensity,
    external_force: ExternalForce,
    ray_caster: RayCaster,
}

impl Default for TankBundle {
    fn default() -> Self {
        Self {
            hover: Hover::default(),
            hold_position: HoldPosition {
                strength: Vec3::new(1.0, 0.0, 1.0),
            },
            rigid_body: RigidBody::Dynamic,
            collider: Collider::cuboid(1.0, 1.0, 1.0),
            collider_density: ColliderDensity(750.0),
            external_force: ExternalForce::default().with_persistence(false),
            ray_caster: RayCaster::new(Vec3::ZERO, Dir3::NEG_Y).with_max_hits(1),
        }
    }
}
