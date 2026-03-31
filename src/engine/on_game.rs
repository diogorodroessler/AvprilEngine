use bevy::{
    ecs::{component::Component, system::Query},
    input::{ButtonInput, keyboard::KeyCode, mouse::MouseButton},
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};

/// Only the Window Screen
#[derive(Component)]
pub struct InGameScreenSceneWorkflow;

impl InGameScreenSceneWorkflow {
    /// Enable/Disable the cursor in the Game Screen
    pub fn draw_cursor(
        mouse: Res<ButtonInput<MouseButton>>,
        key: Res<ButtonInput<KeyCode>>,
        mut cursor: Query<&mut CursorOptions>,
    ) {
        let mut cur_ops = cursor.single_mut().unwrap();

        if key.just_pressed(KeyCode::Escape) {
            cur_ops.visible = true;
            cur_ops.grab_mode = CursorGrabMode::None;
        }

        if mouse.just_pressed(MouseButton::Left) {
            cur_ops.visible = false;
            cur_ops.grab_mode = CursorGrabMode::Locked;
        }
    }
}
