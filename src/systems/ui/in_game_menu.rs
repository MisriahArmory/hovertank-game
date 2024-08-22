use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    constants::menu::NORMAL_BUTTON, key_mappings::in_game_ui::InGameUiAction,
    states::in_game::InGame,
};

pub fn setup_in_game_menu(mut commands: Commands) {
    commands
        .spawn((
            StateScoped(InGame::MenuOpen),
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(400.),
                        height: Val::Px(400.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "In Game Menu",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        });
}

pub fn toggle_in_game_menu(
    query: Query<&ActionState<InGameUiAction>>,
    current_state: Res<State<InGame>>,
    mut next_state: ResMut<NextState<InGame>>,
) {
    for action in query.iter() {
        for pressed_action in action.get_just_pressed() {
            match pressed_action {
                InGameUiAction::ToggleMenu => next_state.set(match current_state.get() {
                    InGame::Running => InGame::MenuOpen,
                    InGame::MenuOpen => InGame::Running,
                }),
            }
        }
    }
}
