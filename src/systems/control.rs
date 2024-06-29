use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{components::LocalPlayer, key_mappings::movement_key_mapping::MoveAction};

/// System to handle input controls.  Looks for an entity with an assigned input mapping
/// and will scan against the controls to determine if the entity should be moved.
pub fn control(query: Query<&ActionState<MoveAction>, With<LocalPlayer>>) {
    for action in &query {
        for pressed_action in action.get_pressed() {
            match pressed_action {
                MoveAction::Left => {
                    info!("I want to move left")
                }
                MoveAction::Right => {
                    info!("I want to move right")
                }
                MoveAction::Forward => {
                    info!("I want to move forward")
                }
                MoveAction::Backward => {
                    info!("I want to move backward")
                }
            }
        }
    }
}
