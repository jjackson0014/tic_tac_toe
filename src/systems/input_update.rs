use bevy::prelude::*;
use crate::resources::user_inputs::GameInput;

pub fn collect_game_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut input: ResMut<GameInput>,
) {
    input.reset();

    let mut delta = IVec2::ZERO;

    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        delta.y += 1;
    }
    if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        delta.y -= 1;
    }
    if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
        delta.x -= 1;
    }
    if keyboard_input.just_pressed(KeyCode::ArrowRight) {
        delta.x += 1;
    }

    input.move_delta = delta;

    if keyboard_input.just_pressed(KeyCode::Space) {
        input.toggle_scale = true;
    }
}
