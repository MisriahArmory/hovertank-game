use avian3d::prelude::*;
use bevy::prelude::*;

use crate::components::Hover;

pub fn hover(
    mut query: Query<(
        &mut ExternalForce,
        &Hover,
        &RayCaster,
        &RayHits,
        &Mass,
        &LinearVelocity,
    )>,
    gravity: Res<Gravity>,
) {
    for (mut force, hover, _, hits, mass, velocity) in query.iter_mut() {
        let Some(hit) = hits.iter().next() else {
            continue;
        };

        if hit.time_of_impact >= hover.target_height {
            continue;
        }

        if velocity.y >= hover.max_hover_speed {
            continue;
        }

        let hover_strength =
            (hover.target_height / hit.time_of_impact).min(hover.max_hover_strength);

        let gravity_force = mass.0 * gravity.0;

        let hover_force = -gravity_force * hover_strength;

        force.apply_force(hover_force);
    }
}
