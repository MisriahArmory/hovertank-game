use bevy::prelude::*;

#[derive(Component, Debug, Default)]
pub struct MovementProperties {
    pub forward_move_strength: f32,
    pub backward_move_strength: f32,
    pub left_move_strength: f32,
    pub right_move_strength: f32,
}
