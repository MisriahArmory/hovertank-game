use bevy::prelude::*;
use leafwing_input_manager::{
    axislike::{AxisType, MouseMotionAxisType},
    prelude::*,
};

#[derive(Actionlike, Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum RotationAction {
    Rotate,
}

impl RotationAction {
    pub fn default_mapping() -> InputMap<RotationAction> {
        let mut input_map = InputMap::default();

        input_map.insert(
            RotationAction::Rotate,
            DualAxis {
                x: SingleAxis {
                    axis_type: AxisType::MouseMotion(MouseMotionAxisType::X),
                    positive_low: 0.001,
                    negative_low: -0.001,
                    inverted: false,
                    sensitivity: 0.05,
                    value: None,
                },
                y: SingleAxis {
                    axis_type: AxisType::MouseMotion(MouseMotionAxisType::Y),
                    positive_low: 0.001,
                    negative_low: -0.001,
                    inverted: false,
                    sensitivity: 0.05,
                    value: None,
                },
                deadzone: DeadZoneShape::Ellipse {
                    radius_x: 0.001,
                    radius_y: 0.001,
                },
            },
        );

        input_map
    }
}
