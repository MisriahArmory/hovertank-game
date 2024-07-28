use bevy::{
    color::palettes::css,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*
};
use iyes_progress::{
    Progress,
    ProgressCounter,
};

use crate::{
    components::loading::{LoadingBar, OnLoading},
    resources::timer::LoadingTimer,
    states::app::AppState,
};


// Time in seconds to complete a custom long-running task.
// If assets are loaded earlier, the current state will not
// be changed until the 'fake long task' is completed (thanks to 'iyes_progress')
const DURATION_LONG_TASK_IN_SECS: f32 = 2.0;
const LOADING_BORDER_COLOR:Srgba = css::RED;
const LOADING_BACKGROUND_COLOR: Srgba = css::DARK_RED;
const COMPLETION_BAR_COLOR: Srgba = css::BLUE;


pub fn setup_loading(
    mut commands: Commands,
) {
    commands.spawn((StateScoped(AppState::Loading), Camera2dBundle::default()));
    commands.spawn((
        StateScoped(AppState::Loading),
        NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ..default()
        },
        OnLoading,
    ))
    .with_children(|parent| {
        // Loading bar frame
        parent.spawn((
            StateScoped(AppState::Loading),
            NodeBundle {
                style: Style {
                    width: Val::Percent(80.),
                    height: Val::Percent(5.),
                    border: UiRect::all(Val::Px(5.)),
                    margin: UiRect::all(Val::Px(5.)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                background_color: LOADING_BACKGROUND_COLOR.into(),
                border_color: LOADING_BORDER_COLOR.into(),
                ..Default::default()
            }
        ))
        // Actual loading bar
        .with_children(|parent_loading_frame| {
            parent_loading_frame.spawn((
                StateScoped(AppState::Loading),
                NodeBundle {
                    style: Style {
                        width: Val::Percent(0.),
                        height: Val::Percent(0.),
                        align_items: AlignItems::Start,
                        ..Default::default()
                    },
                    background_color: COMPLETION_BAR_COLOR.into(),
                    ..Default::default()
                },
                LoadingBar::default(),
            ));
        });

        // Loading text
        parent.spawn((
            StateScoped(AppState::Loading),
            TextBundle::from_section(
                "Loading...",
                TextStyle {
                    ..default()
                }
            ),
            Label
        ));
    });

    commands.insert_resource(LoadingTimer(Timer::from_seconds(DURATION_LONG_TASK_IN_SECS, TimerMode::Once)));
}

pub fn loading(
    time: Res<Time>,
    mut timer: ResMut<LoadingTimer>,
) -> Progress {
    // Placeholder until we have actual assets to load

    let elapsed = timer.elapsed();
    debug!("loading delay time elapsed: {}.{}", elapsed.as_secs(), elapsed.subsec_nanos());

    // This runs too fast, and I don't understand why.
    if timer.tick(time.delta()).finished() {
        info!("Fake loading delay finished");
        // the below informs that the task for progress tracking is complete
        true.into()
    } else {
        // the below informs that the task for progress tracking is still ongoing
        false.into()
    }
}

pub fn print_progress(
    progress: Option<Res<ProgressCounter>>,
    diagnostics: Res<DiagnosticsStore>,
    mut loading_bar_query: Query<(&mut LoadingBar, &mut Style)>,
    mut last_done: Local<u32>,
) {
    if let Some(progress) = progress.map(|counter| counter.progress()) {
        if progress.done > *last_done {
            for (_, mut style) in &mut loading_bar_query {
                info!(
                    "found style object with width: {:?}",
                    style.width,
                );
                // This is the money
                style.width = Val::Percent((progress.total / progress.done) as f32 * 100.0);
                info!(
                    "Calculated width: {}",
                    (progress.total / progress.done) as f32
                );
                info!(
                    "new width for style object: {:?}",
                    style.width,
                );
            }
            *last_done = progress.done;
            info!(
                "[Frame {}] Changed progress: {:?}",
                diagnostics
                    .get(&FrameTimeDiagnosticsPlugin::FRAME_COUNT)
                    .map(|diagnostic| diagnostic.value().unwrap_or(0.))
                    .unwrap_or(0.),
                progress
            );
        }
    }
}
