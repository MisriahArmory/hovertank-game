use bevy::prelude::*;

use crate::Rotor3;

pub trait Project {
    fn project(&self, norm: Self) -> Self;
    fn project_normalized(&self, norm: Self) -> Self;
}

impl Project for Vec3 {
    /// Projects a vector onto the plane with the given normal vector
    fn project(&self, norm: Self) -> Self {
        *self - self.dot(norm) * norm
    }

    /// Projects and normalizes a vector on the plane with the given normal vector
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
