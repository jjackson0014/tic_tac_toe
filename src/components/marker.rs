use crate::prelude::*;

// A component that represents the player in the game
// [TODO]: Consider impact of having multiple players
#[derive(Component)]
pub struct Player;

// Opponents
#[derive(Component)]
pub struct Opponent;

// Toggle-able UI Node
#[derive(Component)]
pub struct DebugUI;

#[derive(Component)]
pub struct DebugText;