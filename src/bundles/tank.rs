use avian3d::prelude::*;
use bevy::prelude::*;

use crate::components::{
    DampenMovement, DampenRotation, Hover, MovementProperties, NeutralizeRoll, RotationProperties,
};

#[derive(Bundle)]
pub struct TankBundle {
    collider: Collider,
    collider_density: ColliderDensity,
    dampen_movement: DampenMovement,
    dampen_rotation: DampenRotation,
    external_force: ExternalForce,
    external_torque: ExternalTorque,
    hover: Hover,
    movement_properties: MovementProperties,
    neutralize_roll: NeutralizeRoll,
    ray_caster: RayCaster,
    rigid_body: RigidBody,
    rotation_properties: RotationProperties,
}

impl Default for TankBundle {
    fn default() -> Self {
        Self {
            collider: Collider::cuboid(1.0, 1.0, 1.0),
            collider_density: ColliderDensity(750.0),
            dampen_movement: DampenMovement {
                strength: Vec3::new(1.0, 0.0, 1.0),
            },
            dampen_rotation: DampenRotation {
                strength: Vec3::new(1.0, 1.0, 1.0) * 6.0,
            },
            external_force: ExternalForce::default().with_persistence(false),
            external_torque: ExternalTorque::default().with_persistence(false),
            hover: Hover::default(),
            movement_properties: MovementProperties {
                forward_move_strength: 4.0,
                backward_move_strength: 4.0,
                left_move_strength: 4.0,
                right_move_strength: 4.0,
            },
            neutralize_roll: NeutralizeRoll {
                strength: Vec3::new(1.0, 1.0, 1.0) * 12.0,
            },
            ray_caster: RayCaster::new(Vec3::ZERO, Dir3::NEG_Y).with_max_hits(1),
            rigid_body: RigidBody::Dynamic,
            rotation_properties: RotationProperties {
                rotation_strength: 12.0,
            },
        }
    }
}
