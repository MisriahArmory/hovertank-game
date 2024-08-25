use crate::{traits::Project, Rotor3};
use avian3d::prelude::*;
use bevy::prelude::*;

use crate::components::NeutralizeRoll;

pub fn neutralize_roll(
    mut query: Query<(&mut ExternalTorque, &NeutralizeRoll, &Transform, &Inertia)>,
) {
    for (mut torque, neutralize_roll, transform, inertia) in query.iter_mut() {
        let right = Vec3::from(transform.right());
        let right_xz = right.project_normalized(Vec3::Y);
        let roll_angle = right_xz.angle_between(right);
        if roll_angle == 0.0 {
            continue;
        }

        let target_rotation = Rotor3::from_rotation_arc(right, right_xz);
        let (axis, angle) = target_rotation.to_axis_angle();
        let neutralization_torque = inertia.mul_vec3(axis * angle) * neutralize_roll.strength;
        torque.apply_torque(neutralization_torque);
    }
}
