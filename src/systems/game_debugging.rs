// game_state_debug.rs
use crate::prelude::*;
use crate::resources::game_states::GameState;

// Prints a message whenever the game state changes
pub fn debug_game_state(state: Res<State<GameState>>) {
    if state.is_changed() {
        println!("Game state changed to: {:?}", state.get());
    }
}