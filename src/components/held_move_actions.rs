use bevy::prelude::*;
use std::collections::HashSet;

use crate::key_mappings::movement_key_mapping::MoveAction;

#[derive(Default, Component)]
pub struct HeldMoveActions {
    pub held_move_actions: HashSet<MoveAction>,
}
