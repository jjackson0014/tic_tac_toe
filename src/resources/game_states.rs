use crate::prelude::*;

// Define the game states for the application
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    HoldingScreen, // Initial state before the game starts
    Loading, // State when the game is loading resources or maps
    Overworld, // Main game state where the player navigates the overworld
    //Battle, // State when the player is engaged in a battle
}
