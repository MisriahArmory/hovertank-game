use avian3d::prelude::{AngularVelocity, Inertia, LinearVelocity, Mass};
use bevy::prelude::*;

use crate::{constants::orbit::ORBIT_ANGLE_EPS, Rotor3};

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
        let rotor =
            Rotor3::from_rotation_arc(relative_translation_direction_in_plane, forward_in_plane);
        let mut orbit_rotor = Rotor3::IDENTITY;
        if relative_angle > ORBIT_ANGLE_EPS {
            orbit_rotor.smooth_nudge(&rotor, orbit_speed, rate);
        } else {
            orbit_rotor = rotor;
        }

        let orbit_translation = orbit_rotor.mul_vec3(relative_translation);
        other - orbit_translation
    }
}

pub trait Dampen<T> {
    type Output;
    /// Computes the dampening term for a kind of motion
    fn dampen(&self, inertia: &T, strength: Vec3) -> Self::Output;
}

impl Dampen<Inertia> for AngularVelocity {
    type Output = Vec3;
    fn dampen(&self, inertia: &Inertia, strength: Vec3) -> Self::Output {
        inertia.mul_vec3(-self.0) * strength
    }
}

impl Dampen<Mass> for LinearVelocity {
    type Output = Vec3;
    fn dampen(&self, inertia: &Mass, strength: Vec3) -> Self::Output {
        inertia.0 * -self.0 * strength
    }
}
