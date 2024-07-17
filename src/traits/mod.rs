use bevy::prelude::*;

pub trait Project {
    fn project(&self, norm: Self) -> Self;
    fn project_normalized(&self, norm: Self) -> Self;
}

impl Project for Vec3 {
    /// Projects a vector onto the plane with the given normal vector
    fn project(&self, norm: Self) -> Self {
        *self - self.dot(norm) / norm.dot(norm) * norm
    }

    /// Projects and normalizes a vector on the plane with the given normal vector
    fn project_normalized(&self, norm: Self) -> Self {
        self.project(norm).normalize()
    }
}
