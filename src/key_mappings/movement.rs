use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum MoveAction {
    Left,
    Right,
    Forward,
    Backward,
}

impl MoveAction {
    pub fn default_mapping() -> InputMap<MoveAction> {
        let mut input_map = InputMap::default();

        input_map.insert(Self::Left, KeyCode::KeyA);
        input_map.insert(Self::Right, KeyCode::KeyD);
        input_map.insert(Self::Forward, KeyCode::KeyW);
        input_map.insert(Self::Backward, KeyCode::KeyS);

        input_map
    }
}
