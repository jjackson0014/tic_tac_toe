use crate::prelude::*;
use crate::resources::{
    user_inputs::GameInput,
    game_states::GameState,
};

// Collects user input for the game, including movement, state changes, and other actions
// This sytem is for tracking the keyboard input - the plan is that some inputs may need to be queued and handled separately
// so there are different systems for handling different types of input
pub fn collect_game_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut input: ResMut<GameInput>,
    game_state: Res<State<GameState>>,
) {
    input.reset();

    match game_state.get() {
        GameState::MainMenu => {
            
        }
        GameState::Overworld => {
            // Movement input
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
        }
        _ => {}
    }

    // State/Map input
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        input.chosen_map = "testmapS".to_string();
    }
    if keyboard_input.just_pressed(KeyCode::Digit2) {
        input.chosen_map = "testmapM".to_string();
    }
    if keyboard_input.just_pressed(KeyCode::Digit3) {
        input.chosen_map = "testmapL".to_string();
    }

    // Debug UI Node
    if keyboard_input.just_pressed(KeyCode::KeyD) {
        input.toggle_debug_node = true;
    }

    // Window input
    if keyboard_input.just_pressed(KeyCode::Space) {
        input.toggle_scale = true;
    }

    // Misc input
    if keyboard_input.just_pressed(KeyCode::Escape) {
        println!("Pressed Escape - Exiting game");
        std::process::exit(0);
    }
}