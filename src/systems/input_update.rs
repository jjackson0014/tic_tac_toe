use crate::prelude::*;
use crate::resources::{
    user_inputs::GameInput,
    map_load_request::LoadedMap,
    game_states::GameState,
    // map_load_request::MapLoadState,
};
use crate::components::marker::DebugUI;

// Collects user input for the game, including movement, state changes, and other actions
// This sytem is for tracking the keyboard input - the plan is that some inputs may need to be queued and handled separately
// so there are different systems for handling different types of input
pub fn collect_game_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut input: ResMut<GameInput>,
    // current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut current_map: ResMut<LoadedMap>,
    mut debug_ui_query: Query<&mut Node, With<DebugUI>>,
) {
    input.reset();

    // State/Map input
    if keyboard_input.just_pressed(KeyCode::KeyS) {
        // println!("Pressed S - Loading map: maps/testmapS");
        *current_map = LoadedMap("testmapS".to_string());
        next_state.set(GameState::Loading);
    }
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        // println!("Pressed M - Loading map: maps/testmapM");
        *current_map = LoadedMap("testmapM".to_string());
        next_state.set(GameState::Loading);
    }
    if keyboard_input.just_pressed(KeyCode::KeyL) {
        // println!("Pressed L - Loading map: maps/testmapL");
        *current_map = LoadedMap("testmapL".to_string());
        next_state.set(GameState::Loading);
    }

    // Debug UI Node
    if keyboard_input.just_pressed(KeyCode::KeyD) {
        println!("Pressed D - Toggling debug UI visibility");
        for mut node in debug_ui_query.iter_mut() {
            if node.display == Display::None {
                println!("Showing UI Node");
                node.display = Display::Flex; // Show the debug UI
            } else {
                println!("Hiding UI Node");
                node.display = Display::None; // Hide the debug UI
            }
        }
    }

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