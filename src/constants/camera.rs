use std::{
    f32::consts::{FRAC_PI_4, FRAC_PI_6, PI, TAU},
    sync::LazyLock,
};

pub const CAMERA_FOLLOW_SPEED: f32 = 3.0;
pub const CAMERA_FOLLOW_DISTANCE_XZ: f32 = 5.0;
pub const CAMERA_FOLLOW_HEIGHT: f32 = 2.0;
pub const CAMERA_FOLLOW_ROTATION_SPEED: f32 = 4.0 * TAU;
pub const CAMERA_COMFORT_ZONE_PITCH: f32 = FRAC_PI_6;
pub const CAMERA_MAX_PITCH: f32 = FRAC_PI_4;
pub const CAMERA_MIN_PITCH: f32 = -FRAC_PI_4;
pub const CAMERA_YAW_SPEED: f32 = 1.0;
pub const CAMERA_PITCH_SPEED: f32 = 1.0;
pub const CAMERA_MAX_YAW_SPEED: f32 = PI;
pub const CAMERA_MAX_PITCH_SPEED: f32 = PI;

pub static CAMERA_FOLLOW_DISTANCE: LazyLock<f32> = LazyLock::<f32>::new(|| {
    (CAMERA_FOLLOW_DISTANCE_XZ.powf(2.0) + CAMERA_FOLLOW_HEIGHT.powf(2.0)).sqrt()
});
