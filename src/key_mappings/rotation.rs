use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum RotationAction {
    Rotate,
}

impl RotationAction {
    pub fn default_mapping() -> InputMap<RotationAction> {
        InputMap::default()
            .with_dual_axis(
                RotationAction::Rotate,
                MouseMove::default().sensitivity_x(0.02).sensitivity_y(0.05),
            )
            .with_dual_axis(
                RotationAction::Rotate,
                GamepadStick::RIGHT
                    .inverted_y()
                    .with_deadzone_x_symmetric(0.01)
                    .with_deadzone_y_symmetric(0.01)
                    .sensitivity_x(1.0)
                    .sensitivity_y(1.0),
            )
    }
}

impl Actionlike for RotationAction {
    fn input_control_kind(&self) -> InputControlKind {
        InputControlKind::DualAxis
    }
}
