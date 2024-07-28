use bevy::prelude::*;

use crate::Rotor3;

pub trait Project {
    fn project(&self, norm: Self) -> Self;
    fn project_normalized(&self, norm: Self) -> Self;
}

impl Project for Vec3 {
    /// Projects a vector onto the plane with the given normal vector
    #[inline]
    fn project(&self, norm: Self) -> Self {
        *self - self.dot(norm) * norm
    }

    /// Projects and normalizes a vector on the plane with the given normal vector
    #[inline]
    fn project_normalized(&self, norm: Self) -> Self {
        self.project(norm).normalize()
    }
}

pub trait StableInterpolate: Clone {
    fn interpolate_stable(&self, other: &Self, t: f32) -> Self;
    fn interpolate_stable_assign(&mut self, other: &Self, t: f32) {
        *self = self.interpolate_stable(other, t);
    }
    fn smooth_nudge(&mut self, target: &Self, decay_rate: f32, delta: f32) {
        self.interpolate_stable_assign(target, 1.0 - f32::exp(-decay_rate * delta));
    }
}

impl StableInterpolate for Vec3 {
    #[inline]
    fn interpolate_stable(&self, other: &Self, t: f32) -> Self {
        self.lerp(*other, t)
    }
}

impl StableInterpolate for Rotor3 {
    #[inline]
    fn interpolate_stable(&self, other: &Self, t: f32) -> Self {
        self.slerp(*other, t)
    }
}

pub trait Orbit {
    fn orbit(
        &self,
        other: Self,
        forward: Self,
        orbital_plane_normal: Vec3,
        orbit_speed: f32,
        rate: f32,
    ) -> Self;
}

impl Orbit for Vec3 {
    fn orbit(
        &self,
        other: Self,
        forward: Self,
        orbital_plane_normal: Vec3,
        orbit_speed: f32,
        rate: f32,
    ) -> Self {
        let relative_translation = other - *self;
        let relative_translation_direction_in_plane =
            relative_translation.project_normalized(orbital_plane_normal);

        let forward_in_plane = forward.project_normalized(orbital_plane_normal);
        let relative_angle =
            relative_translation_direction_in_plane.angle_between(forward_in_plane);
        let orbit_speed = relative_angle * orbit_speed;
        // We need to flip the arc here since we are usually behind the focus object and our rotation
        // is applied from the forward direction.
        let rotor =
            Rotor3::from_rotation_arc(relative_translation_direction_in_plane, forward_in_plane);
        let mut orbit_rotor = Rotor3::IDENTITY;
        orbit_rotor.smooth_nudge(&rotor, orbit_speed, rate);

        let orbit_translation = orbit_rotor.mul_vec3(relative_translation);
        other - orbit_translation
    }
}
