use bevy::{diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, prelude::*};
use iyes_progress::{Progress, ProgressCounter};

use crate::{
    components::loading::OnLoading, resources::timer::LoadingTimer,
};


// Time in seconds to complete a custom long-running task.
// If assets are loaded earlier, the current state will not
// be changed until the 'fake long task' is completed (thanks to 'iyes_progress')
const DURATION_LONG_TASK_IN_SECS: f32 = 2.0;


pub fn setup_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((StateScoped(AppState::Loading), Camera2dBundle::default()));
    let icon = asset_server.load("branding/icon.png");
    commands
        .spawn((
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
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Px(200.0),
                    ..default()
                },
                image: UiImage::new(icon),
                ..default()
            });
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
    mut last_done: Local<u32>,
) {
    if let Some(progress) = progress.map(|counter| counter.progress()) {
        if progress.done > *last_done {
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
