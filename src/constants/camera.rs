use std::f32::consts::{FRAC_PI_6, FRAC_PI_8, PI};

pub const CAMERA_LOOK_AHEAD_DISTANCE: f32 = 5.0;
pub const CAMERA_FOLLOW_SPEED: f32 = 3.0;
pub const CAMERA_FOLLOW_DISTANCE: f32 = 5.0;
pub const CAMERA_FOLLOW_HEIGHT: f32 = 1.0;
pub const CAMERA_FOLLOW_PITCH_ROTATION_SPEED: f32 = 16.0;
pub const CAMERA_FOLLOW_YAW_ROTATION_SPEED: f32 = 403.0;
pub const CAMERA_COMFORT_ZONE_ADJUSTMENT_SPEED: f32 = 0.03;
pub const CAMERA_COMFORT_ZONE_MAX_PITCH: f32 = FRAC_PI_8 / 3.0;
pub const CAMERA_COMFORT_ZONE_MIN_PITCH: f32 = -FRAC_PI_6 / 3.0;
pub const CAMERA_COMFORT_ZONE_TIME: f64 = 3.0;
pub const CAMERA_MAX_PITCH: f32 = FRAC_PI_6 / 2.0;
pub const CAMERA_MIN_PITCH: f32 = -FRAC_PI_6 / 2.0;
pub const CAMERA_MAX_YAW_SPEED: f32 = 2.0 / 3.0 * PI;
pub const CAMERA_MAX_PITCH_SPEED: f32 = 1.0;
pub const CAMERA_FOLLOW_SNAP_DISTANCE: f32 = 0.01;
