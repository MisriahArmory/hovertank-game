use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum MoveAction {
    Move,
    Left,
    Right,
    Forward,
    Backward,
}

impl MoveAction {
    pub fn default_mapping() -> InputMap<MoveAction> {
        let mut input_map = InputMap::default().with_dual_axis(
            MoveAction::Move,
            GamepadStick::LEFT
                .with_circle_deadzone(0.01)
                .sensitivity_x(1.0)
                .sensitivity_y(1.0),
        );

        input_map.insert(Self::Left, KeyCode::KeyA);
        input_map.insert(Self::Right, KeyCode::KeyD);
        input_map.insert(Self::Forward, KeyCode::KeyW);
        input_map.insert(Self::Backward, KeyCode::KeyS);

        input_map
    }
}

impl Actionlike for MoveAction {
    fn input_control_kind(&self) -> InputControlKind {
        match self {
            Self::Move => InputControlKind::DualAxis,
            _ => InputControlKind::Button,
        }
    }
}
