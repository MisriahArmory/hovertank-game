use avian3d::prelude::*;
use bevy::prelude::*;

use crate::components::{DampenMovement, DampenRotation, Hover, RotationProperties};

#[derive(Bundle)]
pub struct TankBundle {
    hover: Hover,
    dampen_movement: DampenMovement,
    dampen_rotation: DampenRotation,
    rigid_body: RigidBody,
    collider: Collider,
    collider_density: ColliderDensity,
    external_force: ExternalForce,
    external_torque: ExternalTorque,
    ray_caster: RayCaster,
    rotation_properties: RotationProperties,
}

impl Default for TankBundle {
    fn default() -> Self {
        Self {
            hover: Hover::default(),
            dampen_movement: DampenMovement {
                strength: Vec3::new(1.0, 0.0, 1.0),
            },
            dampen_rotation: DampenRotation {
                strength: Vec3::new(1.0, 1.0, 1.0),
            },
            rigid_body: RigidBody::Dynamic,
            collider: Collider::cuboid(1.0, 1.0, 1.0),
            collider_density: ColliderDensity(750.0),
            external_force: ExternalForce::default().with_persistence(false),
            external_torque: ExternalTorque::default().with_persistence(false),
            ray_caster: RayCaster::new(Vec3::ZERO, Dir3::NEG_Y).with_max_hits(1),
            rotation_properties: RotationProperties {
                rotation_strength: 12.0,
            },
        }
    }
}
