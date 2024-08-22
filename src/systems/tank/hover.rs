use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{components::Hover, constants::physics::PHYSICS_DOWNWARD_FORCE_RANGE};

pub fn hover(
    mut set: ParamSet<(
        Query<(
            &mut ExternalForce,
            &Hover,
            &RayCaster,
            &RayHits,
            &Mass,
            &LinearVelocity,
        )>,
        Query<&mut ExternalForce>,
    )>,
    gravity: Res<Gravity>,
) {
    let mut hover_query = set.p0();
    let mut entities_underneath = Vec::new();
    for (mut force, hover, _ray, hits, mass, velocity) in hover_query.iter_mut() {
        let Some(hit) = hits.iter().next() else {
            continue;
        };

        if hit.time_of_impact >= hover.target_height && hit.time_of_impact >= hover.braking_height {
            continue;
        }

        if velocity.y >= hover.max_hover_speed {
            continue;
        }

        let hover_strength = if hit.time_of_impact > hover.target_height
            && hit.time_of_impact < hover.braking_height
        {
            (hover.braking_height / hit.time_of_impact).min(hover.max_brake_strength)
        } else {
            (hover.target_height / hit.time_of_impact).min(hover.max_hover_strength)
        };

        let gravity_force = mass.0 * gravity.0;

        let hover_force = -gravity_force * hover_strength;

        force.apply_force(hover_force);

        // Apply downward force on anything below the hovering entity.
        // We will model the force as proportional to the square of the distance
        if hit.time_of_impact > PHYSICS_DOWNWARD_FORCE_RANGE {
            continue;
        }

        let force_attenuation = 1.0 / hit.time_of_impact.powf(2.0);

        entities_underneath.push((hit.entity, -hover_force * force_attenuation));
    }

    let mut force_entities = set.p1();

    for (entity, force) in entities_underneath {
        let Ok(mut f) = force_entities.get_mut(entity) else {
            continue;
        };

        f.apply_force(force);
    }
}
