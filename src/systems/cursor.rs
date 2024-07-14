use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn grab_cursor(mut q_windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = q_windows.single_mut();
    primary_window.cursor.grab_mode = CursorGrabMode::Locked;
    primary_window.cursor.visible = false;
}

#[cfg(target_os = "windows")]
fn grab_cursor(mut q_windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = q_windows.single_mut();
    primary_window.cursor.grab_mode = CursorGrabMode::Confined;
    primary_window.cursor.visible = false;

    let center = Vec2::new(primary_window.width() / 2.0, primary_window.height() / 2.0);
    primary_window.set_cursor_position(Some(center));
}

pub fn release_cursor(mut q_windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = q_windows.single_mut();
    primary_window.cursor.grab_mode = CursorGrabMode::None;
    primary_window.cursor.visible = true;
}
