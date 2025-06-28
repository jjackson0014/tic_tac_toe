use crate::prelude::*;

/// Resource storing the player's current grid position. 
#[derive(Resource)]
struct PlayerPosition {
    x: i32,
    y: i32,
}